use na::Vector2;

struct OccupancyGrid {
    storage: Box<[bool]>,
    x_cells: usize,
    y_cells: usize,
    origin: Vector2<f32>,
    resolution: f32,
}

impl OccupancyGrid {
    pub fn new(
        x_cells: usize,
        y_cells: usize,
        origin: Vector2<f32>,
        resolution: f32,
    ) -> OccupancyGrid {
        OccupancyGrid {
            storage: vec![false; x_cells * y_cells].into_boxed_slice(),
            x_cells,
            y_cells,
            origin,
            resolution,
        }
    }

    pub fn cell(&self, x: usize, y: usize) -> &bool {
        assert!(x < self.x_cells && y < self.y_cells);
        &self.storage[y * self.x_cells + x]
    }

    pub fn cell_mut(&mut self, x: usize, y: usize) -> &mut bool {
        assert!(x < self.x_cells && y < self.y_cells);
        &mut self.storage[y * self.x_cells + x]
    }

    pub fn position_to_cell(&self, pos: &Vector2<f32>) -> (usize, usize) {
        assert!(self.origin.x <= pos.x && self.origin.y <= pos.y);
        let real_size = self.real_size();
        assert!(pos.x <= (self.origin.x + real_size.x) && pos.y <= (self.origin.y + real_size.y));

        let cell_f = (pos - self.origin) / self.resolution;
        (cell_f.x as usize, cell_f.y as usize)
    }

    pub fn size(&self) -> (usize, usize) {
        (self.x_cells, self.y_cells)
    }

    pub fn real_size(&self) -> Vector2<f32> {
        Vector2::new(
            self.x_cells as f32 * self.resolution,
            self.y_cells as f32 * self.resolution,
        )
    }

    pub fn origin(&self) -> Vector2<f32> {
        self.origin
    }

    pub fn resolution(&self) -> f32 {
        self.resolution
    }
}
