use na::Vector2;

pub struct Static2dTree<const LEAF_CAP: usize, const CELLS_X: usize, const CELLS_Y: usize> {
    cells: [[[Vector2<u32>; LEAF_CAP]; CELLS_Y]; CELLS_X],
}

impl<const LEAF_CAP: usize, const CELLS_X: usize, const CELLS_Y: usize>
    Static2dTree<LEAF_CAP, CELLS_X, CELLS_Y>
{
    pub fn empty() -> Self {
        Static2dTree {
            cells: [[[Vector2::<u32>::new(0, 0); LEAF_CAP]; CELLS_Y]; CELLS_X],
        }
    }

    pub fn add_point(&mut self, point: Vector2<u32>) -> bool {}

    pub fn closest_point(&self, query: Vector2<u32>) -> Option<usize> {}
}
