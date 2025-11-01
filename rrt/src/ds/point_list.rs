use std::ops::Index;

use na::SVector;

pub trait PointList<const DIMS: usize>: Index<usize, Output = SVector<f32, DIMS>> {
    fn add_point(&mut self, point: SVector<f32, DIMS>) -> bool;
    fn len(&self) -> usize;
}
