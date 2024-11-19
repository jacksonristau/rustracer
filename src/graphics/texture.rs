use zune_jpeg::JpegDecoder;
use jpeg_encoder;
use core::fmt;
use std::fs::read;
use crate::graphics::color::Color;

/*
const GAUSSIAN_KERNEL: [f64; 9] = [
    1.0/16.0, 1.0/8.0, 1.0/16.0,
    1.0/8.0, 1.0/4.0, 1.0/8.0,
    1.0/16.0, 1.0/8.0, 1.0/16.0
];
*/

pub struct Texture {
    pub width : i32,
    pub height : i32,
    pub data : Vec<Color>,
    pub filename : String,
}

impl Texture {
    pub fn new(filename : &str) -> Self {
        let file_contents = read(filename).unwrap();
        let mut decoder = JpegDecoder::new(&file_contents);
        let data = decoder.decode().unwrap();
        let image_info = decoder.info().unwrap();
        // convert Vec<u8> to Vec<Color>
        let color_data = data
            .chunks(3)
            .map(|c| Color::new(c[0] as f64 / 255.0, c[1] as f64 / 255.0, c[2] as f64 / 255.0))
            .collect::<Vec<Color>>();
        Texture {
            width : image_info.width as i32,
            height : image_info.height as i32,
            filename : filename.to_string(),
            data : color_data
        }
    }

    pub fn get_pixel(&self, mut u : f64, mut v : f64) -> Color {
        let x;
        let y;

        if u > 1.0 {
            u = u.fract();
        }
        if v > 1.0 {
            v = v.fract();
        }

        y = v * (self.height - 1) as f64;
        x = u * (self.width - 1) as f64;
        if y * self.width as f64 + x >= self.data.len() as f64 {
            panic!("invalid texture coordinates");
        }

        self.data[(y * self.width as f64 + x) as usize]
    }

    pub fn write_to_file(&self, filename : &str) {
        let mut image = Vec::new();
        for color in &self.data {
            image.push((color.r * 255.0) as u8);
            image.push((color.g * 255.0) as u8);
            image.push((color.b * 255.0) as u8);
        }
        let encoder = jpeg_encoder::Encoder::new_file(filename, 100).unwrap();
        encoder.encode(&image, self.width as u16, self.height as u16, jpeg_encoder::ColorType::Rgb).unwrap();
    }

    pub fn pad(&self) -> Self {
        let mut padded_data = Vec::new();
        for y in 0..self.height + 2 {
            for x in 0..self.width + 2 {
                let color = if x == 0 || y == 0 || x == self.width + 1 || y == self.height + 1 {
                    Color::new(0.0, 0.0, 0.0) // Padding color
                } else {
                    self.data[((y - 1) * self.width + (x - 1)) as usize]
                };
                padded_data.push(color);
            }
        }
        Texture {
            width : self.width + 2,
            height : self.height + 2,
            filename : self.filename.clone(),
            data : padded_data
        }
    }

    fn apply_kernel(&self, kernel: &Vec<f64>, x: i32, y: i32, kernel_size: usize) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        let half_k = (kernel_size / 2) as i32;
        let mut sum_r = 0.0;
        let mut sum_g = 0.0;
        let mut sum_b = 0.0;

        for ky in -half_k..=half_k {
            for kx in -half_k..=half_k {
                let px = (x + kx).clamp(0, self.width - 1);
                let py = (y + ky).clamp(0, self.height - 1);
                let pixel = self.data[(py * self.width + px) as usize];
                let kernel_value = kernel[((ky + half_k) * kernel_size as i32 + (kx + half_k)) as usize];

                sum_r += pixel.r * kernel_value;
                sum_g += pixel.g * kernel_value;
                sum_b += pixel.b * kernel_value;
            }
        }

        color.r = sum_r;
        color.g = sum_g;
        color.b = sum_b;
        color
    }

    pub fn filter(&self, kernel : &Vec<f64>, kernel_size: usize) -> Self {
        let mut filtered_data = Vec::new();
        let padded_texture = self.pad();
        println!("Applying filter to texture: {}", self.filename);
        for y in 1..padded_texture.height-1 {
            for x in 1..padded_texture.width-1 {
                let color = padded_texture.apply_kernel(&kernel, x, y, kernel_size);
                filtered_data.push(color);
            }
        }
        Texture {
            width : self.width,
            height : self.height,
            filename : self.filename.clone(),
            data : filtered_data
        }
    }
}

impl fmt::Display for Texture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Texture width: {}, height: {}, data length: {}", self.width, self.height, self.data.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_creation() {
        let texture = Texture::new("images/walz.jpeg");
        assert!(texture.width > 0);
        assert!(texture.height > 0);
        assert!(!texture.data.is_empty());
    }

    #[test]
    fn test_invalid_texture_coordinates() {
        let texture = Texture::new("images/walz.jpeg");
        assert!(texture.get_pixel(2.0, 2.0) == texture.get_pixel(0.0, 0.0)) // This should panic
    }

    #[test]
    fn test_valid_texture_coordinates() {
        let texture = Texture::new("images/walz.jpeg");
        texture.get_pixel(0.5, 0.5); // This should not panic
    }

    #[test]
    fn test_texture_wrapping_coordinates() {
        let texture = Texture::new("images/walz.jpeg");
        texture.get_pixel(1.5, 1.5); // This should wrap around and not panic
    }
}
