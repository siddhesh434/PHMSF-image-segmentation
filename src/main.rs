mod graph;
mod parallel;

use image::{Rgb, RgbImage};
use graph::image::load_graph_from_image;
use crate::graph::kruskal::Kruskal;
use crate::parallel::algo::Algo;
use std::time::Instant;

fn main() {
    let image_path = "static/4.1.05.tiff";
    let threshold = 13f32;
    let contrast = -3f32;

    let start_time = Instant::now();

    match load_graph_from_image(image_path) {
        Ok(graph) => {
            let mut algo = Kruskal::new(&graph, threshold, contrast);
            let ind = algo.apply_threshold();

            let segmented_image1 = algo.relabel(&graph.pixel);
            algo.apply(ind);

            let mut img1 = RgbImage::new(graph.dimensions.0, graph.dimensions.1);

            for y in 0..graph.dimensions.1 {
                for x in 0..graph.dimensions.0 {
                    let (r, g, b) = segmented_image1[y as usize][x as usize];
                    img1.put_pixel(y, x, Rgb([255 - r, 255 - g, 255 - b]));
                }
            }
            img1.save("segmented_image_serial.png").expect("Failed to save the image");
            let elapsed_time = start_time.elapsed();
            println!("Time taken for sequential algorithm: {:?}", elapsed_time);
            println!("Sequential algorithm applied successfully.");
        },
        Err(e) => {
            eprintln!("Failed to load graph from image: {}", e);
        }
    }

    let start_time_parallel = Instant::now();

    match parallel::graph::load_graph_from_image_with_tiles(image_path, 64, 64, threshold, contrast) {
        Ok(mut graph) => {
            let algo: Algo = Algo::new();
            let segmented_image = algo.apply(&mut graph);
            let elapsed_time_parallel = start_time_parallel.elapsed();

            let mut img = RgbImage::new(graph.width as u32, graph.height as u32);
            for y in 0..graph.height {
                for x in 0..graph.width {
                    let (r, g, b) = segmented_image[y][x];
                    img.put_pixel(x as u32, y as u32, Rgb([255 - r, 255 - g, 255 - b]));
                }
            }
            img.save("segmented_image_parallel.png").expect("Failed to save the image");
            println!("Time taken for parallel algorithm: {:?}", elapsed_time_parallel);
            println!("Parallel algorithm applied successfully.");
        },
        Err(e) => {
            eprintln!("Failed to load graph from image with tiles: {}", e);
        }
    }

}
