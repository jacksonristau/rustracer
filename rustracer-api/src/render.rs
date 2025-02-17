use rustracer_core::{raytracer, scene::Scene, graphics::vec_writer::VecWriter};
use std::sync::Arc;

pub const FILENAME: &str = "../images/output.jpg";

pub fn handle_render(scene: Scene) -> Vec<u8> {
    let raytracer = Arc::new(raytracer::Raytracer::new(scene));
    let px_width = raytracer.scene.resolution.0;
    let px_height = raytracer.scene.resolution.1;

    let pixel_map = raytracer.trace_rays();

    let mut image = Vec::new();
    for color in pixel_map {
        image.push((color.r * 255.0) as u8);
        image.push((color.g * 255.0) as u8);
        image.push((color.b * 255.0) as u8);
    }
    let mut jpeg_bytes = Vec::new();
    {
        let writer = VecWriter::new(&mut jpeg_bytes);
        let encoder = jpeg_encoder::Encoder::new(writer, 100);
        encoder.encode(&image, px_width as u16, px_height as u16, jpeg_encoder::ColorType::Rgb).unwrap();
    }
    jpeg_bytes
}