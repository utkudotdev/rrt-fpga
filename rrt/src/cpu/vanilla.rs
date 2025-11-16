use na::Vector2;
use nalgebra as na;
use rand::Rng;

use super::alg::raytrace;
use crate::shared::alg::dfs;
use crate::shared::ds::grid::OccupancyGrid;
use crate::shared::ds::nn_index::NNIndex;
use crate::{RRTAlgorithm, RRTParameters, RRTResult};

pub struct VanillaRRT;

impl<NN: NNIndex<2>> RRTAlgorithm<NN> for VanillaRRT {
    fn run(
        &self,
        start: &Vector2<f32>,
        goal: &Vector2<f32>,
        grid: &OccupancyGrid,
        params: &RRTParameters,
    ) -> RRTResult<NN> {
        let mut nn_index = NN::empty();

        let start_idx = 0;
        assert!(nn_index.add_point(*start));

        let mut tree: Vec<Vec<usize>> = vec![Vec::new(); params.num_points + 1];
        let mut found = false;
        let mut end_idx = 0;

        let mut rng = rand::thread_rng();

        while nn_index.len() < params.num_points {
            let mut conf = Vector2::<f32>::zeros();
            for i in 0..conf.len() {
                conf[i] = rng.gen_range(params.min_bound[i]..=params.max_bound[i]);
            }

            let nearest_idx = nn_index.closest_point(conf).unwrap();
            let nearest = nn_index[nearest_idx];

            let direction = (conf - nearest).normalize();
            let in_between = nearest + direction * params.move_dist;

            if !(params.min_bound <= in_between && in_between < params.max_bound) {
                continue;
            }

            if raytrace::is_segment_occupied(&nearest, &in_between, &grid) {
                continue;
            }

            if !nn_index.add_point(in_between) {
                continue;
            }
            let new_idx = nn_index.len() - 1;

            tree[nearest_idx].push(new_idx);

            if (in_between - goal).norm_squared() < params.sq_dist_tol {
                found = true;
                end_idx = new_idx;
                break;
            }
        }

        let path = if found {
            let mut path_vec = Vec::new();
            dfs::dfs(&tree, start_idx, end_idx, &mut path_vec);
            Some(path_vec)
        } else {
            None
        };

        RRTResult {
            points: nn_index,
            tree,
            path,
        }
    }
}
