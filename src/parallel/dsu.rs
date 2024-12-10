use std::mem::swap;
use std::sync::{Arc, RwLock};

/// Thread-safe Disjoint Set Union
pub struct DisjointSetUnion {
    parent: Vec<RwLock<usize>>,
    smallest_edge: Vec<RwLock<f32>>,
    pub(crate) size: Vec<RwLock<i32>>,
    pub(crate) credit: Vec<RwLock<f32>>,
    pub threshold: f32,
    contrast: f32,
}

impl DisjointSetUnion {
    pub fn new(nodes: usize, threshold: f32, contrast: f32) -> Arc<Self> {
        Arc::new(DisjointSetUnion {
            parent: (0..nodes).map(|i| RwLock::new(i)).collect(),
            smallest_edge: (0..nodes).map(|_| RwLock::new(f32::NAN)).collect(),
            size: (0..nodes).map(|_| RwLock::new(1)).collect(),
            credit: (0..nodes).map(|_| RwLock::new(f32::NAN)).collect(),
            threshold,
            contrast,
        })
    }

    pub fn find(&self, node: usize) -> usize {
        let mut current = node;

        loop {
            let parent = *self.parent[current].read().unwrap();
            if parent == current {
                break;
            }

            // Path compression: Point current node to its grandparent
            let grandparent = *self.parent[parent].read().unwrap();
            *self.parent[current].write().unwrap() = grandparent;

            current = parent;
        }

        current
    }

    pub fn compute_credit(&self, node: usize, weight: f32) -> f32 {
        let par = self.find(node);

        let min_perimeter: f32 = (4.0 * std::f32::consts::PI * *self.size[par].read().unwrap() as f32).sqrt();

        let contrast = {
            let smallest_edge = *self.smallest_edge[node].read().unwrap();
            if smallest_edge.is_nan() {
                weight
            } else {
                smallest_edge
            }
        } - 2f32 * self.contrast;

        contrast * min_perimeter
    }

    pub fn union(&self, u: usize, v: usize, weight: f32) -> Option<(usize, usize)> {
        let mut u = self.find(u);
        let mut v = self.find(v);

        if u != v {
            let credit_u = {
                let mut credit = self.credit[u].write().unwrap();
                if credit.is_nan() {
                    *credit = self.compute_credit(u, weight);
                }
                *credit
            };

            let credit_v = {
                let mut credit = self.credit[v].write().unwrap();
                if credit.is_nan() {
                    *credit = self.compute_credit(v, weight);
                }
                *credit
            };

            let credit = credit_u.min(credit_v);

            if credit > weight {
                if *self.size[u].read().unwrap() < *self.size[v].read().unwrap() {
                    swap(&mut u, &mut v);
                }

                // Perform union
                *self.parent[v].write().unwrap() = u;

                let mut size_u = self.size[u].write().unwrap();
                let size_v = *self.size[v].read().unwrap();
                *size_u += size_v;

                let mut smallest_edge_u = self.smallest_edge[u].write().unwrap();
                let smallest_edge_v = *self.smallest_edge[v].read().unwrap();
                *smallest_edge_u = smallest_edge_u.min(smallest_edge_v);

                let mut credit_u = self.credit[u].write().unwrap();
                *credit_u = credit - weight;
                Some((u, v))
            } else {
                None
            }

        } else {
            None
        }
    }

    pub fn union_threshold(&self, u: usize, v: usize, weight: f32) -> Option<(usize, usize)>  {
        let mut u = self.find(u);
        let mut v = self.find(v);

        if u != v && weight < self.threshold {
            if *self.size[u].read().unwrap() < *self.size[v].read().unwrap() {
                swap(&mut u, &mut v);
            }

            // Perform union
            *self.parent[v].write().unwrap() = u;

            let mut size_u = self.size[u].write().unwrap();
            let size_v = *self.size[v].read().unwrap();
            *size_u += size_v;

            let mut smallest_edge_u = self.smallest_edge[u].write().unwrap();
            let smallest_edge_v = *self.smallest_edge[v].read().unwrap();
            *smallest_edge_u = smallest_edge_u.min(smallest_edge_v);

            return Some((u, v));
        }
        None
    }
}
