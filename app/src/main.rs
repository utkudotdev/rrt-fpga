use nalgebra::Vector2;
use rrt::cpu::ds::kdtree::KdTree;
use rrt::cpu::vanilla::VanillaRRT;
use rrt::shared::ds::grid::OccupancyGrid;
use rrt::shared::ds::point_list::PointList;
use rrt::{RRTAlgorithm, RRTParameters, RRTResult};
use sfml::graphics::{
    CircleShape, Color, FloatRect, PrimitiveType, RectangleShape, RenderTarget, RenderWindow,
    Shape, Transformable, Vertex, View,
};
use sfml::system::Vector2f;
use sfml::window::{Event, Style};

const START: Vector2<f32> = Vector2::new(0.0, 0.0);
const GOAL: Vector2<f32> = Vector2::new(0.05, 0.0);
const NUM_POINTS: usize = 1000;
const MOVE_DIST: f32 = 0.01;
const MIN_BOUND: Vector2<f32> = Vector2::new(-0.1, -0.1);
const MAX_BOUND: Vector2<f32> = Vector2::new(0.1, 0.1);
const SQ_GOAL_TOL: f32 = 0.0001;
const GRID_RESOLUTION: f32 = 0.01;
const CIRCLE_RADIUS: f32 = 5.0;

fn run_vis<PL: PointList<2>>(
    result: &RRTResult<PL>,
    real_goal: &Vector2<f32>,
    grid: &OccupancyGrid,
) {
    let mut window = RenderWindow::new(
        (800, 600),
        "RRT Visualization",
        Style::DEFAULT,
        &Default::default(),
    )
    .expect("Failed to create window");
    window.set_framerate_limit(60);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::Resized { width, height } => {
                    let visible_area = FloatRect::new(0.0, 0.0, width as f32, height as f32);
                    window.set_view(&View::from_rect(visible_area).expect("Failed to create view"));
                }
                _ => {}
            }
        }

        let window_size = window.size();
        let scale_x = window_size.x as f32 / grid.real_size().x;
        let scale_y = window_size.y as f32 / grid.real_size().y;

        window.clear(Color::WHITE);

        // Draw occupancy grid
        let (x_cells, y_cells) = grid.size();
        for y in 0..y_cells {
            for x in 0..x_cells {
                if *grid.cell(x, y) {
                    let mut rect = RectangleShape::new();
                    rect.set_size(Vector2f::new(
                        grid.resolution() * scale_x,
                        grid.resolution() * scale_y,
                    ));
                    rect.set_position(Vector2f::new(
                        (x as f32 * grid.resolution()) * scale_x,
                        window_size.y as f32 - ((y + 1) as f32 * grid.resolution()) * scale_y,
                    ));
                    rect.set_fill_color(Color::rgba(128, 128, 128, 255)); // Gray
                    window.draw(&rect);
                }
            }
        }

        // Draw tree
        for i in 0..result.points.len() {
            let p1 = result.points[i];
            for &j in &result.tree[i] {
                let p2 = result.points[j];

                let line = [
                    Vertex::with_pos_color(
                        Vector2f::new(
                            (p1[0] - grid.origin().x) * scale_x,
                            window_size.y as f32 - (p1[1] - grid.origin().y) * scale_y,
                        ),
                        Color::rgba(200, 200, 200, 255),
                    ),
                    Vertex::with_pos_color(
                        Vector2f::new(
                            (p2[0] - grid.origin().x) * scale_x,
                            window_size.y as f32 - (p2[1] - grid.origin().y) * scale_y,
                        ),
                        Color::rgba(220, 220, 220, 255),
                    ),
                ];
                window.draw_primitives(&line, PrimitiveType::LINES, &Default::default());
            }
        }

        // Draw path
        if let Some(path) = &result.path {
            for i in 0..path.len() - 1 {
                let p1 = result.points[path[i]];
                let p2 = result.points[path[i + 1]];

                let line = [
                    Vertex::with_pos_color(
                        Vector2f::new(
                            (p1[0] - grid.origin().x) * scale_x,
                            window_size.y as f32 - (p1[1] - grid.origin().y) * scale_y,
                        ),
                        Color::BLUE,
                    ),
                    Vertex::with_pos_color(
                        Vector2f::new(
                            (p2[0] - grid.origin().x) * scale_x,
                            window_size.y as f32 - (p2[1] - grid.origin().y) * scale_y,
                        ),
                        Color::BLUE,
                    ),
                ];
                window.draw_primitives(&line, PrimitiveType::LINES, &Default::default());
            }
        }

        // Draw points
        for i in 0..result.points.len() {
            let p = result.points[i];
            let mut circle = CircleShape::new(CIRCLE_RADIUS, 30);

            let screen_x = (p[0] - grid.origin().x) * scale_x;
            let screen_y = window_size.y as f32 - (p[1] - grid.origin().y) * scale_y;

            circle.set_origin(Vector2f::new(CIRCLE_RADIUS, CIRCLE_RADIUS));
            circle.set_position(Vector2f::new(screen_x, screen_y));

            if i == 0 {
                // Assuming start_idx is always 0
                circle.set_fill_color(Color::RED);
            } else {
                circle.set_fill_color(Color::BLACK);
            }
            window.draw(&circle);
        }

        // Draw true goal
        let mut end_circle = CircleShape::new(CIRCLE_RADIUS, 30);
        let end_screen_x = (real_goal.x - grid.origin().x) * scale_x;
        let end_screen_y = window_size.y as f32 - (real_goal.y - grid.origin().y) * scale_y;
        end_circle.set_origin(Vector2f::new(CIRCLE_RADIUS, CIRCLE_RADIUS));
        end_circle.set_position(Vector2f::new(end_screen_x, end_screen_y));
        end_circle.set_fill_color(Color::GREEN);
        window.draw(&end_circle);

        window.display();
    }
}

fn main() {
    let diff = MAX_BOUND - MIN_BOUND;
    let x_cells = (diff.x / GRID_RESOLUTION).ceil() as usize;
    let y_cells = (diff.y / GRID_RESOLUTION).ceil() as usize;

    let mut grid = OccupancyGrid::new(x_cells, y_cells, MIN_BOUND, GRID_RESOLUTION);
    *grid.cell_mut(12, 9) = true;
    *grid.cell_mut(12, 10) = true;
    *grid.cell_mut(12, 11) = true;

    let rrt = VanillaRRT;
    let params = RRTParameters {
        num_points: NUM_POINTS,
        move_dist: MOVE_DIST,
        min_bound: MIN_BOUND,
        max_bound: MAX_BOUND,
        sq_dist_tol: SQ_GOAL_TOL,
    };

    let result: RRTResult<KdTree<2, 16>> = rrt.run(&START, &GOAL, &grid, &params);

    run_vis(&result, &GOAL, &grid);
}
