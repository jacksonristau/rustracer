use std::{fs::File, io::Read, fmt};

//use crate::graphics::texture::Texture;
use crate::graphics::{light::Light, material::Material};
use crate::math::sphere::Sphere;
use crate::math::vector::Vector;
use crate::graphics::color::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Scene {
    pub materials : Vec<Material>,
    pub spheres : Vec<Sphere>,
    pub lights : Vec<Light>,

    pub eye_pos : Vector,
    pub view_dir : Vector,
    pub up_dir : Vector,

    pub hfov : f64,

    // (width, height)
    pub resolution : (i32, i32),
    pub bkg_color : Color,
    pub frustum_width : f64,
    pub parallel : bool,
    pub dc: Color,
    pub alpha : (f64, f64),
    pub dist : (f64, f64),
}

impl Scene {
    pub fn new(materials : Vec<Material>, spheres : Vec<Sphere>, lights : Vec<Light>, eye_pos : Vector, view_dir : Vector, up_dir : Vector, hfov : f64, resolution : (i32, i32), alpha : (f64, f64), dist : (f64, f64), bkg_color : Color, frustum_width : f64, depth_cue : Color, parallel : bool) -> Self {
        Scene {
            materials : materials,
            spheres : spheres,
            lights : lights,
            eye_pos : eye_pos,
            view_dir : view_dir,
            up_dir : up_dir,
            dc: depth_cue,
            alpha : alpha,
            dist : dist,
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

impl fmt::Display for Scene {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "width: {}, height: {}, fov: {}\neye position: {}, view direction: {}, up direction: {}\n# of materials: {}\n# of spheres {}\n",
            self.resolution.0, self.resolution.1, self.hfov, self.eye_pos, self.view_dir, self.up_dir, self.materials.len(), self.spheres.len()
        )
    }
}
