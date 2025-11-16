pub mod ds;

use ds::vec_point_list::VecPointList;

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
        todo!()
    }
}
