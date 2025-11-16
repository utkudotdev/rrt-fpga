use crate::shared::ds::grid::OccupancyGrid;
use na::Vector2;

pub fn is_segment_occupied(a: &Vector2<f32>, b: &Vector2<f32>, grid: &OccupancyGrid) -> bool {
    let (mut cell_x, mut cell_y) = grid.position_to_cell(a);

    // line can be reparameterized as f(t) = t * delta + a where t: [0, 1]
    let mut t = 0.0;
    let delta = b - a;

    loop {
        if *grid.cell(cell_x, cell_y) {
            return true;
        }

        let current = a + t * delta;

        let next_x_intersection = (if delta.x > 0.0 { cell_x + 1 } else { cell_x }) as f32
            * grid.resolution()
            + grid.origin().x;
        let mut remaining_x_t = (next_x_intersection - current.x) / delta.x;

        let next_y_intersection = (if delta.y > 0.0 { cell_y + 1 } else { cell_y }) as f32
            * grid.resolution()
            + grid.origin().y;
        let mut remaining_y_t = (next_y_intersection - current.y) / delta.y;

        // sometimes if delta is 0 it can mess up the sign
        if remaining_x_t.is_infinite() {
            remaining_x_t = remaining_x_t.abs();
        }
        if remaining_y_t.is_infinite() {
            remaining_y_t = remaining_y_t.abs();
        }

        assert!(t >= 0.0);
        assert!(t <= 1.0);
        assert!(remaining_x_t >= 0.0);
        assert!(remaining_y_t >= 0.0);
        // but not necessarily <= 1.0

        let x_increment: isize = if delta.x > 0.0 { 1 } else { -1 };
        let y_increment: isize = if delta.y > 0.0 { 1 } else { -1 };

        if 1.0 - t < remaining_x_t && 1.0 - t < remaining_y_t {
            // we'll reach the end strictly before any intersection
            // means the end is in the current cell which we've already checked, so exit
            break;
        } else if remaining_x_t == remaining_y_t {
            // extremely rare
            // in this case, we can basically choose our path
            // but we want to avoid "leaking" through so we shouldn't do +1 on each
            // if both our blocked we'll collide next iteration anyways
            let next_cell_x = (cell_x as isize + x_increment) as usize;
            if *grid.cell(next_cell_x, cell_y) {
                t += remaining_y_t;
                cell_y = (cell_y as isize + y_increment) as usize;
            } else {
                t += remaining_x_t;
                cell_x = (cell_x as isize + x_increment) as usize;
            }
        } else if remaining_x_t > remaining_y_t {
            t += remaining_y_t;
            cell_y = (cell_y as isize + y_increment) as usize;
        } else {
            t += remaining_x_t;
            cell_x = (cell_x as isize + x_increment) as usize;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use na::vector;

    #[test]
    fn test_within_one_cell() {
        let mut grid = OccupancyGrid::new(2, 2, vector![0.0, 0.0], 1.0);
        *grid.cell_mut(1, 0) = true;
        *grid.cell_mut(0, 1) = true;
        *grid.cell_mut(1, 1) = true;

        assert!(!is_segment_occupied(
            &vector![0.25, 0.25],
            &vector![0.75, 0.75],
            &grid
        ));
    }

    #[test]
    fn test_basic_multiple_cells_unoccupied() {
        let mut grid = OccupancyGrid::new(3, 3, vector![0.0, 0.0], 1.0);
        *grid.cell_mut(0, 2) = true;
        *grid.cell_mut(1, 2) = true;

        assert!(!is_segment_occupied(
            &vector![0.5, 0.5],
            &vector![2.7, 2.3],
            &grid
        ));
    }

    #[test]
    fn test_basic_multiple_cells_occupied() {
        let mut grid = OccupancyGrid::new(3, 3, vector![0.0, 0.0], 1.0);
        *grid.cell_mut(1, 1) = true;

        assert!(is_segment_occupied(
            &vector![0.5, 0.5],
            &vector![2.7, 2.3],
            &grid
        ));
    }

    #[test]
    fn test_negative_delta_x() {
        let mut grid = OccupancyGrid::new(3, 3, vector![0.0, 0.0], 1.0);
        *grid.cell_mut(0, 0) = true;
        *grid.cell_mut(2, 2) = true;

        assert!(!is_segment_occupied(
            &vector![2.5, 0.5],
            &vector![0.5, 2.5],
            &grid
        ));
    }

    #[test]
    fn test_negative_delta_y() {
        let mut grid = OccupancyGrid::new(3, 3, vector![0.0, 0.0], 1.0);
        *grid.cell_mut(0, 0) = true;
        *grid.cell_mut(2, 2) = true;

        assert!(!is_segment_occupied(
            &vector![0.5, 2.5],
            &vector![2.5, 0.5],
            &grid
        ));
    }

    #[test]
    fn test_shifted_origin() {
        let mut grid = OccupancyGrid::new(3, 3, vector![-1.5, -1.5], 1.0);
        *grid.cell_mut(2, 0) = true;
        *grid.cell_mut(0, 2) = true;

        assert!(!is_segment_occupied(
            &vector![-1.0, -1.0],
            &vector![1.0, 1.0],
            &grid
        ));
    }

    #[test]
    fn test_line_through_point_corners_unoccupied() {
        let mut grid = OccupancyGrid::new(3, 3, vector![-1.5, -1.5], 1.0);

        let directions = vec![
            vector![1.5, -1.5],
            vector![1.5, 1.5],
            vector![-1.5, 1.5],
            vector![-1.5, -1.5],
        ];
        let occupied_cells = vec![(1, 0), (2, 1), (1, 2), (0, 1)];

        for (cell_x, cell_y) in occupied_cells {
            *grid.cell_mut(cell_x, cell_y) = true;
            for dir in &directions {
                assert!(!is_segment_occupied(&vector![0.0, 0.0], dir, &grid));
            }
            *grid.cell_mut(cell_x, cell_y) = false;
        }
    }

    #[test]
    fn test_line_through_point_corners_occupied() {
        let mut grid = OccupancyGrid::new(3, 3, vector![-1.5, -1.5], 1.0);
        *grid.cell_mut(1, 0) = true;
        *grid.cell_mut(2, 1) = true;
        *grid.cell_mut(1, 2) = true;
        *grid.cell_mut(0, 1) = true;

        let directions = vec![
            vector![1.5, -1.5],
            vector![1.5, 1.5],
            vector![-1.5, 1.5],
            vector![-1.5, -1.5],
        ];

        for dir in &directions {
            assert!(is_segment_occupied(&vector![0.0, 0.0], dir, &grid));
        }
    }
}
