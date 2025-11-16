use std::ops::Index;

use na::SVector;

use crate::shared::ds::point_list::PointList;

pub struct VecPointList<const DIMS: usize> {
    inner: Vec<SVector<f32, DIMS>>,
}

impl<const DIMS: usize> Index<usize> for VecPointList<DIMS> {
    type Output = SVector<f32, DIMS>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<const DIMS: usize> PointList<DIMS> for VecPointList<DIMS> {
    fn empty() -> Self
    where
        Self: Sized,
    {
        Self { inner: vec![] }
    }

    fn add_point(&mut self, point: SVector<f32, DIMS>) -> bool {
        self.inner.push(point);
        true
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}
