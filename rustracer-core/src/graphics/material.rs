use crate::graphics::color::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Material {
    pub diffuse : Color,
    pub specular : Color,
    pub k_a : f64,
    pub k_d : f64,
    pub k_s : f64,
    pub alpha : f64,
    pub index_of_refraction : f64,
    pub n_val : i32,
    pub texture : Option<i32>
}

impl Material {
    pub fn new(diffuse : Color, specular : Color, k_a : f64, k_d : f64, k_s : f64, alpha : f64, index_of_refraction : f64, n_val : i32, texture : Option<i32>) -> Self {
        Material {
            diffuse : diffuse,
            specular : specular,
            k_a : k_a,
            k_d : k_d,
            k_s : k_s,
            alpha : alpha,
            index_of_refraction : index_of_refraction,
            n_val : n_val,
            texture : texture
        }
    }
}