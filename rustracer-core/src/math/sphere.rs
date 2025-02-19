use std::f32;
use std::cmp::PartialEq;
use std::fmt;
use serde::{Deserialize, Serialize};
use super::vector::Vector;

#[derive(Copy, Clone, Deserialize, Serialize, Debug)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
    pub material_index: usize,
}

impl Sphere {
    pub fn new(center: Vector, radius: f32, material_index: usize) -> Self {
        Sphere { center: center, radius: radius, material_index: material_index }
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Sphere) -> bool {
        self.center == other.center && f32::abs(self.radius - other.radius) < f32::EPSILON
    }
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "center: {0}, radius: {1:.2}", self.center, self.radius)
    }
}