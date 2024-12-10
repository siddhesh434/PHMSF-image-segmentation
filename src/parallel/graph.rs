use crate::parallel::dsu::DisjointSetUnion;
use image::GenericImageView;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct Edge {
    pub node1: usize,
    pub node2: usize,
    pub weight: f32,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub edges: Vec<Edge>,
    pub border_edges: Vec<Edge>,
    pub index: usize,
    pub border_index: usize,
    pub delay_queue: Vec<Edge>,   // Delayed edges for inter-tile processing
}

pub struct Graph {
    pub tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
    pub tile_width: usize,
    pub tile_height: usize,
    pub regions: Arc<Vec<RwLock<HashMap<usize, bool>>>>,
    pub dsu: Arc<DisjointSetUnion>,
    pub pixel: Vec<Vec<(u8, u8, u8)>>,
}

impl Graph {
    pub fn new(width: usize, height: usize, tile_width: usize, tile_height: usize, threshold: f32, contrast: f32) -> Self {
        let num_tiles = (width / tile_width) * (height / tile_height);
        let tiles = vec![
            Tile {
                edges: Vec::new(),
                delay_queue: Vec::new(),
                index: 0,
                border_edges: vec![],
                border_index: 0,
            };
            num_tiles
        ];
        let dsu = DisjointSetUnion::new(width*height, threshold, contrast);
        let mut v = vec![];
        for _ in 0..num_tiles {
            v.push(RwLock::new(HashMap::new()));
        }
        let regions = Arc::new(v);
        Graph {
            tiles,
            width,
            height,
            tile_width,
            tile_height,
            dsu,
            regions,
            pixel: vec![vec![(0, 0, 0); width]; height],
        }
    }

    pub fn add_edge(&mut self, tile_idx: usize, node1: usize, node2: usize, weight: f32) {
        self.tiles[tile_idx].edges.push(Edge { node1, node2, weight });
    }

    pub fn add_border_edge (&mut self, tile_idx: usize, node1: usize, node2: usize, weight: f32) {
        self.tiles[tile_idx].border_edges.push(Edge { node1, node2, weight });
    }

    pub fn add_region(&mut self, tile_idx: usize, region: usize) {
        self.regions[tile_idx].write().unwrap().insert(region, false);
    }
}

pub fn load_graph_from_image_with_tiles(
    path: &str,
    tile_width: usize,
    tile_height: usize,
    threshold: f32,
    contrast: f32
) -> Result<Graph, Box<dyn std::error::Error>> {
    let image = image::open(path)?;
    let (width, height) = image.dimensions();

    let mut graph = Graph::new(width as usize, height as usize, tile_width, tile_height, threshold, contrast);

    for ux in 0..height {
        for uy in 0..width {
            let x = ux as usize;
            let y = uy as usize;

            let tile_x = x / tile_height;
            let tile_y = y / tile_width;
            let tile_idx = tile_x * (width as usize / tile_width) + tile_y;

            let neighbours: Vec<(i32, i32)> = vec![
                (x as i32 + 1, y as i32),
                (x as i32, y as i32 + 1),
                (x as i32 + 1, y as i32 + 1),
                (x as i32 + 1, y as i32 - 1),
            ];

            let pixel1 = image.get_pixel(uy, ux);
            graph.pixel[x][y] = (pixel1[0], pixel1[1], pixel1[2]);
            let node1 = x * width as usize + y;

            graph.add_region(tile_idx, node1);

            for (nx, ny) in neighbours {
                if nx >= 0
                    && nx < height as i32
                    && ny >= 0
                    && ny < width as i32
                {
                    let pixel2 = image.get_pixel(ny as u32, nx as u32);
                    let node2 = nx as usize * width as usize + ny as usize;

                    let mut weight: f32 = (pixel1[0] as f32 - pixel2[0] as f32).powi(2)
                        + (pixel1[1] as f32 - pixel2[1] as f32).powi(2)
                        + (pixel1[2] as f32 - pixel2[2] as f32).powi(2);
                    weight = weight.sqrt();

                    let neighbour_tile_x = nx as usize / tile_height;
                    let neighbour_tile_y = ny as usize / tile_width;
                    let neighbour_tile_idx = neighbour_tile_x * (width as usize / tile_width) + neighbour_tile_y;

                    if tile_idx == neighbour_tile_idx {
                        graph.add_edge(tile_idx, node1, node2, weight);
                    } else {
                        graph.add_border_edge(tile_idx, node1, node2, weight);
                    }
                }
            }
        }
    }

    Ok(graph)
}
