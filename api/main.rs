use std::env;
use std::sync::Arc;

use rustracer::raytracer;
use rustracer::scene::Scene;

fn main() {
    
}

fn commandline() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <scene file>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    let scene: Scene = Scene::from_file(filename);

    println!("Successfully loaded scene from {}", filename);

    let raytracer = Arc::new(raytracer::Raytracer::new(scene));
    let px_width = raytracer.scene.resolution.0;
    let px_height = raytracer.scene.resolution.1;

    let pixel_map = raytracer.trace_rays();
    

    let filename = &args[1];
    let filename = filename.replace(".json", ".jpg");

    let mut image = Vec::new();
    for color in pixel_map {
        image.push((color.r * 255.0) as u8);
        image.push((color.g * 255.0) as u8);
        image.push((color.b * 255.0) as u8);
    }
    let encoder = jpeg_encoder::Encoder::new_file(&filename, 100).unwrap();
    encoder.encode(&image, px_width as u16, px_height as u16, jpeg_encoder::ColorType::Rgb).unwrap();
    println!("Image saved to {}", filename);
}