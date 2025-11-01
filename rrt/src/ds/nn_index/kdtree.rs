use std::ops::Index;

use na::SVector;
use nalgebra as na;

use crate::ds::point_list::PointList;

use super::NNIndex;

enum Node<const DIMS: usize, const LEAF_CAP: usize> {
    Internal {
        dim: usize,
        val: f32,
        left: Box<Node<DIMS, LEAF_CAP>>,
        right: Box<Node<DIMS, LEAF_CAP>>,
    },
    Leaf {
        point_indices: [usize; LEAF_CAP],
        len: usize,
    },
}

pub struct KdTree<const DIMS: usize, const LEAF_CAP: usize> {
    points: Vec<SVector<f32, DIMS>>,
    root: Node<DIMS, LEAF_CAP>,
}

impl<const DIMS: usize, const LEAF_CAP: usize> KdTree<DIMS, LEAF_CAP> {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            root: Node::Leaf {
                point_indices: [0; LEAF_CAP],
                len: 0,
            },
        }
    }

    fn add_point_to_node(
        points: &[SVector<f32, DIMS>],
        point_idx: usize,
        node: &mut Node<DIMS, LEAF_CAP>,
        depth: usize,
    ) {
        match node {
            Node::Internal {
                dim,
                val,
                left,
                right,
            } => {
                let point = points[point_idx];
                if point[*dim] < *val {
                    Self::add_point_to_node(points, point_idx, left, depth + 1);
                } else {
                    Self::add_point_to_node(points, point_idx, right, depth + 1);
                }
            }
            Node::Leaf { point_indices, len } => {
                if *len < LEAF_CAP {
                    point_indices[*len] = point_idx;
                    *len += 1;
                } else {
                    let dim = depth % DIMS;
                    let mut mean = SVector::<f32, DIMS>::zeros();
                    for i in 0..*len {
                        mean += points[point_indices[i]];
                    }
                    mean += points[point_idx];
                    mean /= (*len + 1) as f32;
                    let val = mean[dim];

                    let mut left = Box::new(Node::Leaf {
                        point_indices: [0; LEAF_CAP],
                        len: 0,
                    });
                    let mut right = Box::new(Node::Leaf {
                        point_indices: [0; LEAF_CAP],
                        len: 0,
                    });

                    let mut indices_to_reinsert = Vec::with_capacity(LEAF_CAP + 1);
                    for i in 0..*len {
                        indices_to_reinsert.push(point_indices[i]);
                    }
                    indices_to_reinsert.push(point_idx);

                    for idx in indices_to_reinsert {
                        let point = points[idx];
                        if point[dim] < val {
                            Self::add_point_to_node(points, idx, &mut left, depth + 1);
                        } else {
                            Self::add_point_to_node(points, idx, &mut right, depth + 1);
                        }
                    }

                    *node = Node::Internal {
                        dim,
                        val,
                        left,
                        right,
                    };
                }
            }
        }
    }

    fn closest_point_in_node(
        &self,
        point: SVector<f32, DIMS>,
        node: &Node<DIMS, LEAF_CAP>,
        best_dist_sq: &mut f32,
        best_idx: &mut usize,
        depth: usize,
    ) {
        match node {
            Node::Internal {
                dim,
                val,
                left,
                right,
            } => {
                let dim_dist = point[*dim] - val;
                if dim_dist < 0. {
                    self.closest_point_in_node(point, left, best_dist_sq, best_idx, depth + 1);
                    if dim_dist.powi(2) < *best_dist_sq {
                        self.closest_point_in_node(point, right, best_dist_sq, best_idx, depth + 1);
                    }
                } else {
                    self.closest_point_in_node(point, right, best_dist_sq, best_idx, depth + 1);
                    if dim_dist.powi(2) < *best_dist_sq {
                        self.closest_point_in_node(point, left, best_dist_sq, best_idx, depth + 1);
                    }
                }
            }
            Node::Leaf { point_indices, len } => {
                for i in 0..*len {
                    let idx = point_indices[i];
                    let dist_sq = (self.points[idx] - point).norm_squared();
                    if dist_sq < *best_dist_sq {
                        *best_dist_sq = dist_sq;
                        *best_idx = idx;
                    }
                }
            }
        }
    }
}

impl<const DIMS: usize, const LEAF_CAP: usize> Index<usize> for KdTree<DIMS, LEAF_CAP> {
    type Output = SVector<f32, DIMS>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}

impl<const DIMS: usize, const LEAF_CAP: usize> PointList<DIMS> for KdTree<DIMS, LEAF_CAP> {
    fn add_point(&mut self, point: SVector<f32, DIMS>) -> bool {
        let point_idx = self.points.len();
        self.points.push(point);
        Self::add_point_to_node(&self.points, point_idx, &mut self.root, 0);
        true
    }

    fn len(&self) -> usize {
        self.points.len()
    }
}

impl<const DIMS: usize, const LEAF_CAP: usize> NNIndex<DIMS> for KdTree<DIMS, LEAF_CAP> {
    fn closest_point(&self, point: SVector<f32, DIMS>) -> usize {
        assert!(
            !self.points.is_empty(),
            "Cannot find closest points in an empty set."
        );

        let mut best_dist_sq = f32::INFINITY;
        let mut best_idx = 0;
        self.closest_point_in_node(point, &self.root, &mut best_dist_sq, &mut best_idx, 0);
        best_idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kdtree_closest_point() {
        let mut tree = KdTree::<2, 4>::new();
        tree.add_point(SVector::from([2., 3.]));
        tree.add_point(SVector::from([5., 4.]));
        tree.add_point(SVector::from([9., 6.]));
        tree.add_point(SVector::from([4., 7.]));
        tree.add_point(SVector::from([8., 1.]));
        tree.add_point(SVector::from([7., 2.]));

        let query_point = SVector::from([9., 2.]);
        let closest_idx = tree.closest_point(query_point);
        assert_eq!(closest_idx, 4);

        let query_point = SVector::from([1., 1.]);
        let closest_idx = tree.closest_point(query_point);
        assert_eq!(closest_idx, 0);
    }
}
