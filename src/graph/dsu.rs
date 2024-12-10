use std::mem::swap;

pub struct DisjointSetUnion {
    parent: Vec<usize>,
    smallest_edge: Vec<f32>,
    size: Vec<i32>,
    credit: Vec<f32>,
    pub threshold: f32,
    contrast: f32,
}

impl DisjointSetUnion {
    pub(crate) fn new (nodes: usize, threshold: f32, contrast: f32) -> DisjointSetUnion {
        let mut parent: Vec<usize> = vec![0; nodes ];
        let smallest_edge: Vec<f32> = vec![f32::NAN; nodes];
        let size: Vec<i32> = vec![1; nodes];
        let credit: Vec<f32> = vec![f32::NAN; nodes];
        for i in 0..nodes {
            parent[i]=i;
        }
        DisjointSetUnion {
            parent,
            smallest_edge,
            size,
            credit,
            threshold,
            contrast,
        }
    }

    pub fn find (&mut self, node: usize) -> usize {
        if self.parent[node] == node {
            return node;
        }
        self.parent[node] = self.find(self.parent[node]);
        self.parent[node]
    }

    pub fn credit (&mut self, node: usize, weight: f32) -> f32{
        let par = self.find(node);
        let mut min_perimeter: f32 = 4.0 * std::f32::consts::PI * self.size[par] as f32;
        min_perimeter = min_perimeter.sqrt();
        let mut contrast = 0f32;

        if self.smallest_edge[node].is_nan() {
            contrast = weight;
        } else {
            contrast = self.smallest_edge[node];
        }

        contrast = contrast - 2f32*(self.contrast);

        contrast*min_perimeter
    }

    pub fn union (&mut self, u: usize, v: usize, weight: f32) {
        let mut u = self.find(u);
        let mut v = self.find(v);
        if u != v {

            if self.credit[u].is_nan() {
                self.credit[u] = self.credit(u, weight);
            }
            if self.credit[v].is_nan() {
                self.credit[v] = self.credit(v, weight);
            }
            let credit = self.credit[u].min(self.credit[v]);
            if credit > weight {
                if self.size[u] < self.size[v] {
                    swap (&mut u, &mut v);
                }
                self.parent[v] = u;
                self.size[u] += self.size[v];
                self.smallest_edge[u] = self.smallest_edge[u].min(self.smallest_edge[v]);
                self.credit[u] = credit-weight;
            }
        }
    }

    pub fn union_threshold (&mut self, u: usize, v: usize, weight: f32) {
        let mut u = self.find(u);
        let mut v = self.find(v);
        if u != v {
            if weight < self.threshold {
                if self.size[u] < self.size[v] {
                    swap (&mut u, &mut v);
                }
                self.parent[v] = u;
                self.size[u] += self.size[v];
                self.smallest_edge[u] = self.smallest_edge[u].min(self.smallest_edge[v]);
            }
        }
    }
}