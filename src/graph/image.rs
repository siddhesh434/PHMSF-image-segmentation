use image::GenericImageView;
// Generate graph from an image
use crate::graph::graph::{Graph};

pub fn load_graph_from_image(path: &str) -> Result<Graph, Box<dyn std::error::Error>> {
    let image = image::open(path)?;

    let (width, height) = image.dimensions();

    let mut graph = Graph::new(width * height, width, height); // Ensure the graph is initialized with the correct size

    for ux in 0..height {
        for uy in 0..width {
            let x = ux as i32;
            let y = uy as i32;
            let neighbours: Vec<(i32, i32)> = vec![(x + 1, y), (x, y + 1), (x + 1, y + 1), (x + 1, y - 1)];

            let pixel1 = image.get_pixel(ux, uy);
            graph.set_pixel(pixel1, ux, uy);

            for i in neighbours {
                if (i.0 < height as i32 && i.0 >= 0) && (i.1 < width as i32 && i.1 >= 0) {
                    let pixel2 = image.get_pixel(i.0 as u32, i.1 as u32);
                    let mut weight: f32 = (pixel1[0] as f32 - pixel2[0] as f32).powf(2f32) + (pixel1[1] as f32 - pixel2[1] as f32).powf(2f32) + (pixel1[2] as f32 - pixel2[2] as f32).powf(2f32);
                    weight = weight.sqrt();
                    graph.add_edge(ux * width + uy, (i.0 as u32) * width + i.1 as u32, weight);
                }
            }
        }
    }

    Ok(graph)
}