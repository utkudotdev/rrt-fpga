use na::Vector2;
use nalgebra as na;
use rand::Rng;

use super::alg::raytrace;
use crate::cpu::ds::kdtree::KdTree;
use crate::shared::alg::dfs;
use crate::shared::ds::grid::OccupancyGrid;
use crate::shared::ds::point_list::PointList;
use crate::{RRTAlgorithm, RRTParameters, RRTResult};

pub struct VanillaRRT;

impl RRTAlgorithm<KdTree<2, 16>> for VanillaRRT {
    fn run(
        &self,
        start: &Vector2<f32>,
        goal: &Vector2<f32>,
        grid: &OccupancyGrid,
        params: &RRTParameters,
    ) -> RRTResult<KdTree<2, 16>> {
        let mut kd_tree = KdTree::empty();

        let start_idx = 0;
        assert!(kd_tree.add_point(*start));

        let mut tree: Vec<Vec<usize>> = vec![Vec::new(); params.num_points + 1];
        let mut found = false;
        let mut end_idx = 0;

        let mut rng = rand::thread_rng();

        while kd_tree.len() < params.num_points {
            let mut conf = Vector2::<f32>::zeros();
            for i in 0..conf.len() {
                conf[i] = rng.gen_range(params.min_bound[i]..=params.max_bound[i]);
            }

            let nearest_idx = kd_tree.closest_point(conf).unwrap();
            let nearest = kd_tree[nearest_idx];

            let direction = (conf - nearest).normalize();
            let in_between = nearest + direction * params.move_dist;

            if !(params.min_bound <= in_between && in_between < params.max_bound) {
                continue;
            }

            if raytrace::is_segment_occupied(&nearest, &in_between, &grid) {
                continue;
            }

            if !kd_tree.add_point(in_between) {
                continue;
            }
            let new_idx = kd_tree.len() - 1;

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
            points: kd_tree,
            tree,
            path,
        }
    }
}
