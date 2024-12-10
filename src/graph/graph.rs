use image::Rgba;

pub struct Graph {
    pub dimensions: (u32, u32),
    pub pixel: Vec<Vec<(u8, u8, u8)>>,
    pub nodes: u32,                         // number of nodes
    pub adj_list: Vec<Vec<(u32, f32)>>,     // (node, weight)
}
impl Graph {
    pub fn new(nodes: u32, width: u32, height: u32) -> Graph {
        Graph {
            dimensions: (height, width),
            nodes,
            adj_list: vec![Vec::new(); nodes as usize],
            pixel: vec![vec![(0, 0, 0); width as usize]; height as usize],
        }
    }

    pub fn add_edge(&mut self, u: u32, v: u32, w: f32) {
        self.adj_list[u as usize].push((v, w));
        self.adj_list[v as usize].push((u, w));
    }

    pub fn set_pixel (&mut self, pixel: Rgba<u8>, x: u32, y: u32) {
        self.pixel[x as usize][y as usize] = (pixel[0], pixel[1], pixel[2]);
    }
}