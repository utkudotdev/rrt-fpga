use na::SVector;
use std::ops::Index;

pub trait PointList<const DIMS: usize>: Index<usize, Output = SVector<f32, DIMS>> {
    fn empty() -> Self
    where
        Self: Sized;
    fn add_point(&mut self, point: SVector<f32, DIMS>) -> bool;
    fn len(&self) -> usize;
}
