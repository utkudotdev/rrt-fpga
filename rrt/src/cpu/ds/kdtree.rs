use std::ops::Index;

use na::SVector;
use nalgebra as na;

use crate::shared::ds::point_list::PointList;

const MIN_RADIUS_SQ: f32 = 1e-6 * 1e-6;

enum Node<const LEAF_CAP: usize> {
    Split {
        split_dim: usize,
        split_val: f32,
        left: Box<Node<LEAF_CAP>>,
        right: Box<Node<LEAF_CAP>>,
    },
    Leaf {
        point_indices: [usize; LEAF_CAP],
        len: usize,
    },
}

pub struct KdTree<const DIMS: usize, const LEAF_CAP: usize> {
    points: Vec<SVector<f32, DIMS>>,
    root: Node<LEAF_CAP>,
}

impl<const DIMS: usize, const LEAF_CAP: usize> KdTree<DIMS, LEAF_CAP> {
    fn add_point_to_node(
        points: &[SVector<f32, DIMS>],
        point_idx: usize,
        node: &mut Node<LEAF_CAP>,
    ) -> bool {
        match node {
            Node::Split {
                split_dim: dim,
                split_val: val,
                left,
                right,
            } => {
                let point = points[point_idx];
                if point[*dim] < *val {
                    Self::add_point_to_node(points, point_idx, left)
                } else {
                    Self::add_point_to_node(points, point_idx, right)
                }
            }
            Node::Leaf { point_indices, len } => {
                let new_point = points[point_idx];
                let is_too_close = point_indices[..*len]
                    .iter()
                    .map(|&i| points[i])
                    .any(|p| (new_point - p).norm_squared() < MIN_RADIUS_SQ);

                if is_too_close {
                    return false;
                }

                if *len < LEAF_CAP {
                    point_indices[*len] = point_idx;
                    *len += 1;
                    return true;
                }

                let total_points = *len + 1;
                let existing_sum = point_indices[..*len]
                    .iter()
                    .map(|&i| points[i])
                    .sum::<SVector<f32, DIMS>>();
                let mean = (points[point_idx] + existing_sum) / total_points as f32;

                let variance = point_indices[..*len]
                    .iter()
                    .map(|&i| (points[i] - mean))
                    .map(|p| p.component_mul(&p))
                    .sum::<SVector<f32, DIMS>>()
                    / (total_points - 1) as f32;

                let (split_dim, _) = variance.argmax();
                let split_val = mean[split_dim];

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
                    if point[split_dim] < split_val {
                        assert!(Self::add_point_to_node(points, idx, &mut left));
                    } else {
                        assert!(Self::add_point_to_node(points, idx, &mut right));
                    }
                }

                *node = Node::Split {
                    split_dim,
                    split_val,
                    left,
                    right,
                };
                true
            }
        }
    }

    fn closest_point_in_node(
        &self,
        query: SVector<f32, DIMS>,
        node: &Node<LEAF_CAP>,
        best_dist_sq: &mut f32,
        best_idx: &mut usize,
    ) {
        match node {
            Node::Split {
                split_dim,
                split_val,
                left,
                right,
            } => {
                let dim_dist = query[*split_dim] - split_val;
                if dim_dist < 0.0 {
                    self.closest_point_in_node(query, left, best_dist_sq, best_idx);
                    if dim_dist.powi(2) < *best_dist_sq {
                        self.closest_point_in_node(query, right, best_dist_sq, best_idx);
                    }
                } else {
                    self.closest_point_in_node(query, right, best_dist_sq, best_idx);
                    if dim_dist.powi(2) < *best_dist_sq {
                        self.closest_point_in_node(query, left, best_dist_sq, best_idx);
                    }
                }
            }
            Node::Leaf { point_indices, len } => {
                for i in 0..*len {
                    let idx = point_indices[i];
                    let dist_sq = (self.points[idx] - query).norm_squared();
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
    fn empty() -> Self {
        Self {
            points: Vec::new(),
            root: Node::Leaf {
                point_indices: [0; LEAF_CAP],
                len: 0,
            },
        }
    }

    fn add_point(&mut self, point: SVector<f32, DIMS>) -> bool {
        let point_idx = self.points.len();
        self.points.push(point);
        if !Self::add_point_to_node(&self.points, point_idx, &mut self.root) {
            self.points.pop();
            return false;
        }
        true
    }

    fn len(&self) -> usize {
        self.points.len()
    }
}

impl<const DIMS: usize, const LEAF_CAP: usize> KdTree<DIMS, LEAF_CAP> {
    pub fn closest_point(&self, point: SVector<f32, DIMS>) -> Option<usize> {
        if self.points.is_empty() {
            return None;
        }

        let mut best_dist_sq = f32::INFINITY;
        let mut best_idx = 0;
        self.closest_point_in_node(point, &self.root, &mut best_dist_sq, &mut best_idx);
        Some(best_idx)
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::*;

    fn test_random_inserts_and_queries<const LEAF_CAP: usize>(
        count_points: usize,
        count_queries: usize,
    ) {
        let mut rng = rand::thread_rng();
        let mut points = Vec::with_capacity(count_points);
        for _ in 0..count_points {
            points.push(SVector::<f32, 2>::new(
                rng.r#gen::<f32>(),
                rng.r#gen::<f32>(),
            ));
        }

        let mut queries = Vec::with_capacity(count_queries);
        for _ in 0..count_queries {
            queries.push(SVector::<f32, 2>::new(
                rng.r#gen::<f32>(),
                rng.r#gen::<f32>(),
            ));
        }

        let mut tree = KdTree::<2, LEAF_CAP>::empty();
        for p in &points {
            tree.add_point(*p);
        }

        for q in &queries {
            let mut min_dist_sq = f32::INFINITY;
            let mut min_id = 0;

            for (id, p) in points.iter().enumerate() {
                let d_sq = (q - p).norm_squared();
                if d_sq < min_dist_sq {
                    min_dist_sq = d_sq;
                    min_id = id;
                }
            }

            let found_id = tree.closest_point(*q).unwrap();
            assert_eq!(min_id, found_id);
        }
    }

    #[test]
    fn test_kdtree_closest_point() {
        let mut tree = KdTree::<2, 4>::empty();
        tree.add_point(SVector::from([2., 3.]));
        tree.add_point(SVector::from([5., 4.]));
        tree.add_point(SVector::from([9., 6.]));
        tree.add_point(SVector::from([4., 7.]));
        tree.add_point(SVector::from([8., 1.]));
        tree.add_point(SVector::from([7., 2.]));

        let query_point = SVector::from([9., 2.]);
        let closest_idx = tree.closest_point(query_point);
        assert_eq!(closest_idx, Some(4));

        let query_point = SVector::from([1., 1.]);
        let closest_idx = tree.closest_point(query_point);
        assert_eq!(closest_idx, Some(0));
    }

    #[test]
    fn test_add_points() {
        let mut tree = KdTree::<2, 1>::empty();
        assert!(tree.add_point(SVector::from([1.0, 2.0])));
        assert!(!tree.add_point(SVector::from([1.0, 2.0])));
        assert!(tree.add_point(SVector::from([3.0, 2.0])));
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn test_closest_point_larger_leaves() {
        let mut tree = KdTree::<2, 4>::empty();
        tree.add_point(SVector::from([1.0, 2.0]));
        tree.add_point(SVector::from([5.0, 1.0]));
        tree.add_point(SVector::from([-3.0, 8.0]));
        tree.add_point(SVector::from([10.0, 0.2]));
        tree.add_point(SVector::from([-0.9, 4.0]));

        assert_eq!(tree.len(), 5);

        assert_eq!(tree.closest_point(SVector::from([8.0, 0.1])), Some(3));
        assert_eq!(tree.closest_point(SVector::from([-1.0, 3.5])), Some(4));
    }

    #[test]
    fn test_closest_point_small_leaves() {
        let mut tree = KdTree::<2, 1>::empty();
        tree.add_point(SVector::from([1.0, 2.0]));
        tree.add_point(SVector::from([5.0, 1.0]));
        tree.add_point(SVector::from([-3.0, 8.0]));
        tree.add_point(SVector::from([10.0, 0.2]));
        tree.add_point(SVector::from([-0.9, 4.0]));

        assert_eq!(tree.len(), 5);

        assert_eq!(tree.closest_point(SVector::from([8.0, 0.1])), Some(3));
        assert_eq!(tree.closest_point(SVector::from([-1.0, 3.5])), Some(4));
    }

    #[test]
    fn test_closest_point_small_leaf_close_points() {
        let mut tree = KdTree::<2, 1>::empty();
        tree.add_point(SVector::from([5.0, 1.0]));
        tree.add_point(SVector::from([5.0, 1.01]));

        assert_eq!(tree.closest_point(SVector::from([5.0, 0.999])), Some(0));
        assert_eq!(tree.closest_point(SVector::from([5.0, 1.02])), Some(1));
    }

    #[test]
    fn test_random_inserts_leaf_points_1() {
        test_random_inserts_and_queries::<1>(1000, 100);
    }

    #[test]
    fn test_random_inserts_leaf_points_32() {
        test_random_inserts_and_queries::<32>(1000, 100);
    }
}
