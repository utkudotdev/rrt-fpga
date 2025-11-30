pub mod ds;

use ds::vec_point_list::VecPointList;
use na::Vector2;

use crate::fl_model::ds::static_2dtree::Static2dTree;
use crate::shared::ds::grid::OccupancyGrid;
use crate::{RRTAlgorithm, RRTParameters, RRTResult};

pub struct FunctionalModelRRT;

impl RRTAlgorithm<VecPointList<2>> for FunctionalModelRRT {
    fn run(
        &self,
        start: &na::Vector2<f32>,
        goal: &na::Vector2<f32>,
        grid: &OccupancyGrid,
        params: &RRTParameters,
    ) -> RRTResult<VecPointList<2>> {
        let static_tree = Static2dTree::<16, 16, 16>::empty();

        let start_u32 = convert_float_to_fixed(&start, &params.min_bound, &params.max_bound);
        let goal_u32 = convert_float_to_fixed(&goal, &params.min_bound, &params.max_bound);
    }
}

fn convert_float_to_fixed(
    point: &Vector2<f32>,
    min_bound: &Vector2<f32>,
    max_bound: &Vector2<f32>,
) -> Vector2<u32> {
    let bound_size = max_bound - min_bound;
    let point_relative = (point - min_bound).component_div(&bound_size);
    // TODO: there are some float concerns here, and this unwrap is bad
    na::try_convert(point_relative * u32::MAX as f32).unwrap()
}
