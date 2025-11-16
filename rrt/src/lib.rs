pub mod cpu;
pub mod fl_model;
pub mod shared;

extern crate nalgebra as na;

use na::Vector2;
use shared::ds::grid::OccupancyGrid;

use crate::shared::ds::point_list::PointList;

pub struct RRTResult<PL: PointList<2>> {
    pub points: PL,
    pub tree: Vec<Vec<usize>>,
    pub path: Option<Vec<usize>>,
}

pub struct RRTParameters {
    pub num_points: usize,
    pub move_dist: f32,
    pub min_bound: Vector2<f32>,
    pub max_bound: Vector2<f32>,
    pub sq_dist_tol: f32,
}

pub trait RRTAlgorithm<PL: PointList<2>> {
    fn run(
        &self,
        start: &Vector2<f32>,
        goal: &Vector2<f32>,
        grid: &OccupancyGrid,
        params: &RRTParameters,
    ) -> RRTResult<PL>;
}
