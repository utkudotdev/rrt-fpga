pub mod cpu;
pub mod shared;

extern crate nalgebra as na;

use na::Vector2;
use shared::grid::OccupancyGrid;

/// Result of running RRT.
pub struct RRTResult {
    /// A list of points in the tree.
    pub points: Vec<Vector2<f32>>,

    // The actual structure of the tree. If `tree[i][k] = j`, there is an edge from `points[i]` to
    // `points[j]`. Guaranteed to always be a tree.
    pub tree: Vec<Vec<usize>>,

    /// The path from `start` to `goal`, if found. Each element of `path` is an index into
    /// `points`.
    pub path: Option<Vec<usize>>,
}

/// Additional parameters for an `RRTAlgorithm`. Mostly things we don't expect to change frequently,
/// though this isn't a rule that's enforced.
pub struct RRTParameters {
    /// The maximum number of points to add to the tree before giving up.
    pub num_points: usize,

    /// The distance RRT should move towards new points when expanding the tree.
    pub move_dist: f32,

    /// The minimum coordinates of the rectangular region to explore.
    pub min_bound: Vector2<f32>,

    /// The maximum coordinates of the rectangular region to explore.
    pub max_bound: Vector2<f32>,

    /// The square distance to the goal the algorithm needs to achieve to consider a path found.
    pub sq_dist_tol: f32,
}

pub trait RRTAlgorithm {
    /// Runs the `RRTAlgorithm` and returns an `RRTResult` describing the path
    /// found by the algorithm.
    ///
    /// # Arguments
    /// * `start` - The point to start growing the tree from. Must be within `params.min_bound` and
    ///   `params.max_bound`.
    /// * `goal` - The point to find a path to. Once a point closer than `params.sq_dist_tol` in
    ///   squared distance to this point is added to the tree, the path is considered found.
    /// * `grid` - An `OccupancyGrid` representing which areas the path can and cannot traverse. No
    ///   segment in the returned path (if found) will cut an occupied grid cell.
    /// * `params` - Additional parameters for RRT.
    fn run(
        &self,
        start: &Vector2<f32>,
        goal: &Vector2<f32>,
        grid: &OccupancyGrid,
        params: &RRTParameters,
    ) -> RRTResult;
}
