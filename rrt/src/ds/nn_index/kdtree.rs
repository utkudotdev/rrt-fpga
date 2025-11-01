use super::{NNIndex, PointId};
use na::{SMatrix, SVector};
use nalgebra as na;
use std::f32;

const LEAF_SQUARED_TOL: f32 = 1e-7;

type NodeId = usize;

struct SplitNode<const DIMS: usize> {
    lower: NodeId,
    upper: NodeId,
    split_idx: usize,
    split_value: f32,
}

struct LeafNode<const DIMS: usize, const LEAF_SIZE: usize> {
    points: SMatrix<f32, DIMS, LEAF_SIZE>,
    point_ids: [usize; LEAF_SIZE],
    count: usize,
}

impl<const DIMS: usize, const LEAF_SIZE: usize> LeafNode<DIMS, LEAF_SIZE> {
    fn new() -> Self {
        Self {
            points: SMatrix::zeros(),
            point_ids: [0; LEAF_SIZE],
            count: 0,
        }
    }

    fn try_add_point(&mut self, point: &SVector<f32, DIMS>, id: usize) -> bool {
        if self.full() {
            return false;
        }

        self.points.set_column(self.count, point);
        self.point_ids[self.count] = id;
        self.count += 1;

        true
    }

    fn empty(&self) -> bool {
        self.count == 0
    }

    fn full(&self) -> bool {
        self.count == LEAF_SIZE
    }

    fn closest_point(&self, query: &SVector<f32, DIMS>) -> Option<(usize, f32)> {
        if self.empty() {
            return None;
        }

        let mut min_dist_sq = f32::INFINITY;
        let mut result_idx = 0;

        for i in 0..self.count {
            let diff = self.points.column(i) - query;
            let dist_sq = diff.norm_squared();
            if dist_sq < min_dist_sq {
                min_dist_sq = dist_sq;
                result_idx = i;
            }
        }
        Some((result_idx, min_dist_sq))
    }

    fn find_split(&self, to_add: &SVector<f32, DIMS>) -> (usize, f32) {
        let pts_view = self.points.columns(0, self.count);
        let mut new_matrix = pts_view.insert_column(self.points.ncols(), 0.0);
        new_matrix.set_column(new_matrix.ncols() - 1, to_add);

        let mean = new_matrix.column_mean();
        let variance = new_matrix.column_variance();

        let (split_idx, _) = variance.argmax();

        (split_idx, mean[split_idx])
    }
}

enum Node<const DIMS: usize, const LEAF_SIZE: usize> {
    Split(SplitNode<DIMS>),
    Leaf(LeafNode<DIMS, LEAF_SIZE>),
}

pub struct KDTree<const DIMS: usize, const LEAF_SIZE: usize> {
    nodes: Vec<Node<DIMS, LEAF_SIZE>>,
    point_id_to_tree_loc: Vec<usize>,
}

impl<const DIMS: usize, const LEAF_SIZE: usize> KDTree<DIMS, LEAF_SIZE> {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::Leaf(LeafNode::new())],
            point_id_to_tree_loc: Vec::new(),
        }
    }

    fn nn_helper(&self, node_id: NodeId, query: &SVector<f32, DIMS>) -> (usize, f32) {
        match &self.nodes[node_id] {
            Node::Leaf(leaf) => {
                if leaf.empty() {
                    return (0, f32::INFINITY);
                }
                leaf.closest_point(query)
                    .map(|(idx, sq_dist)| (leaf.point_ids[idx], sq_dist))
                    .unwrap_or((0, f32::INFINITY))
            }
            Node::Split(split) => {
                let query_val = query[split.split_idx];
                let (near_child, far_child) = if query_val <= split.split_value {
                    (split.lower, split.upper)
                } else {
                    (split.upper, split.lower)
                };

                let (mut best_id, mut best_dist_sq) = self.nn_helper(near_child, query);

                let dist_to_plane = query_val - split.split_value;
                let dist_sq_to_plane = dist_to_plane * dist_to_plane;

                if dist_sq_to_plane < best_dist_sq {
                    let (candidate_id, candidate_dist_sq) = self.nn_helper(far_child, query);
                    if candidate_dist_sq < best_dist_sq {
                        best_id = candidate_id;
                        best_dist_sq = candidate_dist_sq;
                    }
                }
                (best_id, best_dist_sq)
            }
        }
    }

    fn add_leaf(&mut self) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(Node::Leaf(LeafNode::new()));
        id
    }
}

impl<const DIMS: usize, const LEAF_SIZE: usize> NNIndex<DIMS> for KDTree<DIMS, LEAF_SIZE> {
    fn add_point(&mut self, point: SVector<f32, DIMS>) -> bool {
        let new_id = self.point_id_to_tree_loc.len();
        let mut node_id = 0;

        while let Node::Split(split) = &self.nodes[node_id] {
            node_id = if point[split.split_idx] > split.split_value {
                split.upper
            } else {
                split.lower
            };
        }

        if let Node::Leaf(leaf) = &mut self.nodes[node_id] {
            if let Some((_, sq_dist)) = leaf.closest_point(&point) {
                if sq_dist < LEAF_SQUARED_TOL {
                    return false;
                }
            }

            if leaf.try_add_point(&point, new_id) {
                self.point_id_to_tree_loc
                    .push(node_id * LEAF_SIZE + (leaf.count - 1));
                return true;
            }
        }

        let (split_idx, split_value) = if let Node::Leaf(leaf) = &self.nodes[node_id] {
            leaf.find_split(&point)
        } else {
            unreachable!();
        };

        let lower_id = self.add_leaf();
        let upper_id = self.add_leaf();

        let old_leaf_points;
        let old_leaf_point_ids;
        let old_leaf_count;

        if let Node::Leaf(old_leaf) = &self.nodes[node_id] {
            old_leaf_points = old_leaf.points;
            old_leaf_point_ids = old_leaf.point_ids;
            old_leaf_count = old_leaf.count;
        } else {
            unreachable!();
        }

        for i in 0..old_leaf_count {
            let p = old_leaf_points.column(i);
            let pid = old_leaf_point_ids[i];
            let target_id = if p[split_idx] > split_value {
                upper_id
            } else {
                lower_id
            };

            if let Node::Leaf(target_leaf) = &mut self.nodes[target_id] {
                assert!(target_leaf.try_add_point(&p.into(), pid));
                self.point_id_to_tree_loc[pid] = target_id * LEAF_SIZE + (target_leaf.count - 1);
            }
        }

        let new_target_id = if point[split_idx] > split_value {
            upper_id
        } else {
            lower_id
        };

        if let Node::Leaf(new_target_leaf) = &mut self.nodes[new_target_id] {
            assert!(new_target_leaf.try_add_point(&point, new_id));
            self.point_id_to_tree_loc
                .push(new_target_id * LEAF_SIZE + (new_target_leaf.count - 1));
        }

        self.nodes[node_id] = Node::Split(SplitNode {
            lower: lower_id,
            upper: upper_id,
            split_idx,
            split_value,
        });

        true
    }

    fn get_point(&self, id: PointId) -> SVector<f32, DIMS> {
        let tree_loc = self.point_id_to_tree_loc[id.0];
        if let Node::Leaf(leaf) = &self.nodes[tree_loc / LEAF_SIZE] {
            leaf.points.column(tree_loc % LEAF_SIZE).into()
        } else {
            unreachable!()
        }
    }

    fn size(&self) -> usize {
        self.point_id_to_tree_loc.len()
    }

    fn closest_point(&self, point: SVector<f32, DIMS>) -> PointId {
        PointId(self.nn_helper(0, &point).0)
    }
}
