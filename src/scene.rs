use std::{fs::File, io::Read};

//use crate::graphics::texture::Texture;
use crate::graphics::material::Material;
use crate::math::sphere::Sphere;
use crate::math::vector::Vector;
use crate::graphics::color::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Scene {
    pub materials : Vec<Material>,
    pub spheres : Vec<Sphere>,

    pub eye_pos : Vector,
    pub view_dir : Vector,
    pub up_dir : Vector,

    pub hfov : f64,

    // (width, height)
    pub resolution : (i32, i32),
    pub bkg_color : Color,
    pub frustum_width : f64,
    pub parallel : bool,
}

impl Scene {
    pub fn new(materials : Vec<Material>, spheres : Vec<Sphere>, eye_pos : Vector, view_dir : Vector, up_dir : Vector, hfov : f64, resolution : (i32, i32), bkg_color : Color, frustum_width : f64, parallel : bool) -> Self {
        Scene {
            materials : materials,
            spheres : spheres,
            eye_pos : eye_pos,
            view_dir : view_dir,
            up_dir : up_dir,
            hfov : hfov,
            resolution : resolution,
            bkg_color : bkg_color,
            frustum_width : frustum_width,
            parallel : parallel
        }
    }

    pub fn from_file(filename: &str) -> Self {
        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error opening file {}: {}", filename, e);
                std::process::exit(1);
            }
        };
    
        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            eprintln!("Error reading file {}: {}", filename, e);
            std::process::exit(1);
        }
        match serde_json::from_str(&contents) {
            Ok(scene) => scene,
            Err(e) => {
                eprintln!("Error parsing scene file {}: {}", filename, e);
                std::process::exit(1);
            }
        }
    }

    pub fn write_to_file(&self, filename : &str) {
        let serialized: String = serde_json::to_string(&self).unwrap();
        std::fs::write(filename, serialized).expect("Unable to write file");
    }
}