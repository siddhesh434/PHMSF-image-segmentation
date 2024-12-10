use crate::parallel::graph::Graph;
use rayon::prelude::*;
use std::collections::HashMap;

pub struct Algo;

pub fn get_tile_id (x: usize, y: usize, width: usize, tile_width: usize, tile_height: usize) -> usize {
    let tile_x = x / tile_height;
    let tile_y = y / tile_width;
    tile_x * (width /tile_width) + tile_y
}
impl Algo {

    pub fn new () -> Self {
        Algo
    }
    pub fn threshold_merge (&self, graph: &mut Graph) {
        graph.tiles.par_iter_mut().for_each(|tile| {
            tile.edges.par_sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());
            tile.border_edges.par_sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

            for (ind, edge) in tile.edges.iter().enumerate() {
                if edge.weight <= graph.dsu.threshold {
                    match graph.dsu.union_threshold(edge.node1, edge.node2, edge.weight) {
                        None => {}
                        Some(node) => {
                            let tile = get_tile_id(edge.node1 / graph.width, edge.node1 % graph.width, graph.width, graph.tile_width, graph.tile_height);
                            graph.regions[tile].write().unwrap().remove(&node.1);
                        }
                    }
                } else {
                    tile.index=ind;
                    break;
                }
            }
        });
    }

    pub fn border_edges_merge(&self, graph: &mut Graph) {
        graph.tiles.iter_mut().for_each(|tile| {
            for (ind, edge) in tile.border_edges.iter().enumerate() {
                if edge.weight <= graph.dsu.threshold {
                    match graph.dsu.union_threshold(edge.node1, edge.node2, edge.weight) {
                        None => {}
                        Some(node) => {
                            let tile1 = get_tile_id(edge.node1 / graph.width, edge.node1 % graph.width, graph.width, graph.tile_width, graph.tile_height);
                            let tile2 = get_tile_id(edge.node2 / graph.width, edge.node2 % graph.width, graph.width, graph.tile_width, graph.tile_height);

                            graph.regions[tile1].write().unwrap().remove(&node.1);
                            graph.regions[tile2].write().unwrap().remove(&node.1);

                            if graph.regions[tile1].write().unwrap().contains_key(&node.1) {
                                graph.regions[tile1].write().unwrap().insert(node.0, true);
                            } else {
                                graph.regions[tile2].write().unwrap().insert(node.0, true);
                            }
                        }
                    }
                } else {
                    tile.border_index = ind;
                    break;
                }
            }
        })
    }


    pub fn compute_credit (&self, graph: &mut Graph) {
        graph.regions.par_iter().for_each(|tile| {
            let regions = tile.read().unwrap();
            for i in regions.iter() {
                let credit = graph.dsu.compute_credit(*i.0, 0f32);
                let mut credit_lock = graph.dsu.credit[*i.0].write().unwrap();
                *credit_lock = credit;
            }
        });
    }

    pub fn apply_heuristic (&self, graph: &mut Graph) {
        let width = graph.width;  // Precompute value outside the closure
        let tile_width = graph.tile_width;
        let tile_height = graph.tile_height;

        graph.tiles.par_iter_mut().for_each(|tile| {
            tile.border_index = tile.border_edges.binary_search_by(|a| a.weight.partial_cmp(&graph.dsu.threshold).unwrap()).unwrap_or_else(|ind| ind);
            while tile.index < tile.edges.len() && tile.border_index < tile.border_edges.len() {
                // Use precomputed `width` inside the closure
                if tile.edges[tile.index].weight < tile.border_edges[tile.border_index].weight {
                    let edge = &tile.edges[tile.index];
                    let region1 = graph.dsu.find(edge.node1);
                    let region2 = graph.dsu.find(edge.node2);
                    let tile1 = get_tile_id(edge.node1 / graph.width, edge.node1 % graph.width, graph.width, graph.tile_width, graph.tile_height);
                    if graph.regions[tile1].read().unwrap().get(&region1) == Some(&true)
                        || graph.regions[tile1].read().unwrap().get(&region2) == Some(&true) {
                        tile.delay_queue.push(edge.clone());
                        tile.index += 1;
                        continue;
                    }
                    match graph.dsu.union(edge.node1, edge.node2, edge.weight) {
                        None => {}
                        Some(node) => {
                            graph.regions[tile1].write().unwrap().remove(&node.1);
                        }
                    }
                    tile.index += 1;
                } else {
                    let edge = &tile.border_edges[tile.border_index];
                    let region1 = graph.dsu.find(edge.node1);
                    let region2 = graph.dsu.find(edge.node2);
                    let tile1 = get_tile_id(edge.node1 / width, edge.node1 % width, width, tile_width, tile_height);
                    let tile2 = get_tile_id(edge.node2 / width, edge.node2 % width, width, tile_width, tile_height);
                    graph.regions[tile1].write().unwrap().insert(region1, true);
                    graph.regions[tile2].write().unwrap().insert(region2, true);
                    tile.border_index += 1;
                }
            }
        });
    }

    pub fn delay_queue (&self, graph: &mut Graph) {
        graph.tiles.iter_mut().for_each(|tile| {
            for edge in tile.delay_queue.iter() {
                let tile1 = get_tile_id(edge.node1 / graph.width, edge.node1 % graph.width, graph.width, graph.tile_width, graph.tile_height);
                match graph.dsu.union(edge.node1, edge.node2, edge.weight) {
                    None => {}
                    Some(node) => {
                        graph.regions[tile1].write().unwrap().remove(&node.1);
                    }
                }
            }
        });
    }


    pub fn relabel (&self, graph: &Graph) -> Vec<Vec<(u8, u8, u8)>> {
        let image = &graph.pixel;
        let mut region_colors: HashMap<usize, (f32, f32, f32, i32)> = HashMap::new();

        let height = image.len();
        let width = image[0].len();

        for y in 0..height {
            for x in 0..width {
                let region_id = graph.dsu.find(y * width + x);
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
                let region_id = graph.dsu.find(y * width + x);
                let (r_sum, g_sum, b_sum, count) = region_colors[&region_id];

                let avg_r = (r_sum / count as f32) as u8;
                let avg_g = (g_sum / count as f32) as u8;
                let avg_b = (b_sum / count as f32) as u8;

                result_image[y][x] = (avg_r, avg_g, avg_b);
            }
        }

        result_image
    }

    pub fn apply (&self, graph: &mut Graph) -> Vec<Vec<(u8, u8, u8)>> {
        self.threshold_merge(graph);
        self.border_edges_merge(graph);
        let ans = self.relabel(graph);
        self.compute_credit(graph);
        self.apply_heuristic(graph);
        self.delay_queue(graph);
        ans
    }

}