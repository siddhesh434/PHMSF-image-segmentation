use std::cmp::max;
use std::collections::HashMap;
use crate::graph::dsu::DisjointSetUnion;
use crate::graph::graph::Graph;

pub struct Kruskal {
    edges: Vec<(f32, u32, u32)>,
    dsu: DisjointSetUnion,
}

impl Kruskal {
    pub fn new (graph: &Graph, threshold: f32, contrast: f32) -> Kruskal {
        let mut edges: Vec<(f32, u32, u32)> = Vec::new();
        let mut mxu=0;
        let mut mxv=0;
        for u in 0..graph.nodes {
            mxu = max (u, mxu);
            for v in &graph.adj_list[u as usize] {
                mxv = max (v.0, mxv);
                edges.push((v.1, u, v.0));
            }
        }
        edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        Kruskal {
            edges,
            dsu: DisjointSetUnion::new(graph.nodes as usize, threshold, contrast),
        }
    }

    pub fn apply (&mut self, ind: usize) {
        for i in ind..self.edges.len() {
            let edge = self.edges[i];
            self.dsu.union(edge.1 as usize, edge.2 as usize, edge.0);
        }
    }

    pub fn apply_threshold (&mut self) -> usize {
        for (ind, edge) in self.edges.iter().enumerate() {
            if edge.0 > self.dsu.threshold {
                return ind;
            }
            self.dsu.union_threshold(edge.1 as usize, edge.2 as usize, edge.0);
        }
        self.edges.len()
    }

    pub fn relabel (&mut self, image: &Vec<Vec<(u8, u8, u8)>>) -> Vec<Vec<(u8, u8, u8)>> {
        let mut region_colors: HashMap<usize, (f32, f32, f32, i32)> = HashMap::new();

        let height = image.len();
        let width = image[0].len();

        for y in 0..height {
            for x in 0..width {
                let region_id = self.dsu.find(y * width + x);
                let (r, g, b) = image[y][x];

                let entry = region_colors.entry(region_id).or_insert((0.0, 0.0, 0.0, 0));
                entry.0 += r as f32;
                entry.1 += g as f32;
                entry.2 += b as f32;
                entry.3 += 1;
            }
        }

        let mut result_image = vec![vec![(0, 0, 0); width]; height];

        for y in 0..height {
            for x in 0..width {
                let region_id = self.dsu.find(y * width + x);
                let (r_sum, g_sum, b_sum, count) = region_colors[&region_id];

                let avg_r = (r_sum / count as f32) as u8;
                let avg_g = (g_sum / count as f32) as u8;
                let avg_b = (b_sum / count as f32) as u8;

                result_image[y][x] = (avg_r, avg_g, avg_b);
            }
        }

        result_image
    }
}