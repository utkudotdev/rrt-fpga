use na::Vector2;
use nalgebra as na;
use rand::Rng;

use super::raytrace;
use crate::cpu::kdtree::KdTree;
use crate::shared::dfs;
use crate::shared::grid::OccupancyGrid;
use crate::{RRTAlgorithm, RRTParameters, RRTResult};

pub struct VanillaRRT;

impl RRTAlgorithm for VanillaRRT {
    fn run(
        &self,
        start: &Vector2<f32>,
        goal: &Vector2<f32>,
        grid: &OccupancyGrid,
        params: &RRTParameters,
    ) -> RRTResult {
        let mut kd_tree = KdTree::<2, 16>::empty();

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

        let points = {
            let mut v = Vec::with_capacity(kd_tree.len());
            for i in 0..kd_tree.len() {
                v.push(kd_tree[i]);
            }
            v
        };

        RRTResult { points, tree, path }
    }
}
