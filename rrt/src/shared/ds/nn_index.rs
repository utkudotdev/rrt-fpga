use na::SVector;
use nalgebra as na;

use crate::shared::ds::point_list::PointList;

pub trait NNIndex<const DIMS: usize>: PointList<DIMS> {
    fn closest_point(&self, point: SVector<f32, DIMS>) -> Option<usize>;
}
