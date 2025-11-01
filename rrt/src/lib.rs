mod ds;

extern crate nalgebra as na;
use ds::point_list::PointList;
use na::SVector;

struct RRTResult<const DIMS: usize> {
    points: Box<dyn PointList<DIMS>>,
    tree: Vec<Vec<usize>>,
    path: Option<Vec<usize>>,
}

trait RRTAlgorithm<const DIMS: usize> {
    fn run(
        start: SVector<f32, DIMS>,
        goal: SVector<f32, DIMS>,
        is_edge_free: impl FnMut(SVector<f32, DIMS>, SVector<f32, DIMS>) -> bool,
    ) -> RRTResult<DIMS>;
}
