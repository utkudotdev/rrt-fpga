use sfml::graphics::{
    Color, FloatRect, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable, View,
};
use sfml::system::Vector2f;
use sfml::window::{Event, Key, Style, VideoMode};

fn main() {
    let win_width = 800.0;
    let win_height = 600.0;

    // create the window
    let mut window = RenderWindow::new(
        VideoMode::new(win_width as u32, win_height as u32, 32),
        "rust-sfml — centered square",
        Style::DEFAULT,
        &Default::default(),
    )
    .expect("window creation should succeed");

    window.set_vertical_sync_enabled(true);
    window.set_framerate_limit(60);

    let square_size = 150.0f32;

    let mut square = RectangleShape::new();
    square.set_size(Vector2f::new(square_size, square_size));
    square.set_fill_color(Color::rgb(100, 200, 120));

    // set origin to the center of the shape so positioning centers it
    square.set_origin(Vector2f::new(square_size / 2.0, square_size / 2.0));
    // place in the center of the window
    square.set_position(Vector2f::new(win_width / 2.0, win_height / 2.0));

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::Resized { width, height } => {
                    let visible_area = FloatRect::new(0.0, 0.0, width as f32, height as f32);
                    let view =
                        View::from_rect(visible_area).expect("view creation should not fail");
                    window.set_view(&view);
                }
                _ => {}
            }
        }

        // rendering
        window.clear(Color::BLACK);
        window.draw(&square);
        window.display();
    }
}
