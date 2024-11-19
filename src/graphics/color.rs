use core::fmt;
use::std::ops::{Add, Mul};
use std::cmp::PartialEq;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r: r, g: g, b: b }
    }

    pub fn clamp(&mut self) {
        if self.r > 1.0 {
            self.r = 1.0;
        }
        if self.g > 1.0 {
            self.g = 1.0;
        }
        if self.b > 1.0 {
            self.b = 1.0;
        }
    }

    pub fn to_u8(&self) -> (u8, u8, u8) {
        let r = (self.r * 255.0) as u8;
        let g = (self.g * 255.0) as u8;
        let b = (self.b * 255.0) as u8;
        (r, g, b)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other : Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, s : f64) -> Self {
        Self {
            r: self.r * s,
            g: self.g * s,
            b: self.b * s
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        f64::abs(self.r - other.r) < f64::EPSILON &&
        f64::abs(self.g - other.g) < f64::EPSILON &&
        f64::abs(self.b - other.b) < f64::EPSILON
    }

    fn ne(&self, other: &Color) -> bool {
        f64::abs(self.r - other.r) > f64::EPSILON ||
        f64::abs(self.g - other.g) > f64::EPSILON ||
        f64::abs(self.b - other.b) > f64::EPSILON
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r: {0:.2}, g: {1:.2}, b: {2:.2}", self.r, self.g, self.b)
    }
}

#[test]
fn test_color_new() {
    let color = Color::new(1.0, 1.0, 1.0);
    assert_eq!(color.r, 1.0);
    assert_eq!(color.g, 1.0);
    assert_eq!(color.b, 1.0);
}
#[test]
fn test_color_add() {
    let color1 = Color::new(0.5, 0.5, 0.5);
    let color2 = Color::new(0.2, 0.3, 0.4);
    let result = color1 + color2;
    assert_eq!(result, Color::new(0.7, 0.8, 0.9));
}

#[test]
fn test_color_mul() {
    let color = Color::new(0.5, 0.5, 0.5);
    let result = color * 2.0;
    assert_eq!(result, Color::new(1.0, 1.0, 1.0));
}

#[test]
fn test_color_clamp() {
    let mut color = Color::new(1.5, 0.5, 2.0);
    color.clamp();
    assert_eq!(color, Color::new(1.0, 0.5, 1.0));
}

#[test]
fn test_color_to_u8() {
    let color = Color::new(0.5, 0.5, 0.5);
    let (r, g, b) = color.to_u8();
    assert_eq!((r, g, b), (127, 127, 127));
}

#[test]
fn test_color_partial_eq() {
    let color1 = Color::new(0.5, 0.5, 0.5);
    let color2 = Color::new(0.5, 0.5, 0.5);
    assert_eq!(color1, color2);
}

#[test]
fn test_color_partial_ne() {
    let color1 = Color::new(0.5, 0.5, 0.5);
    let color2 = Color::new(0.5, 0.5, 0.6);
    assert_ne!(color1, color2);
    assert_ne!(color2, color1);
}
