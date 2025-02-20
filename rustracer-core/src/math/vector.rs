use std::f32;
use std::ops::{Add, Sub, Mul, Neg};
use std::cmp::PartialEq;
use std::fmt;
use::serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, Serialize, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vector { x: x, y: y, z: z, w: w }
    }

    pub fn from_vec(v : [f32; 3], w : f32) -> Self {
        Vector { x: v[0], y: v[1], z: v[2], w: w }
    }

    pub fn dot(&self, other: &Vector) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
            w: 0.0
        }
    }

    pub fn distance(&self, other: &Vector) -> f32 {
        f32::sqrt(
            (self.x - other.x) * (self.x - other.x) +
            (self.y - other.y) * (self.y - other.y) +
            (self.z - other.z) * (self.z - other.z)
        )
    }

    pub fn normalize(&mut self) {
        let magnitude : f32 = 1.0 / f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
        self.x *= magnitude;
        self.y *= magnitude;
        self.z *= magnitude;
    }

    pub fn is_normalized(&self) -> bool {
        if f32::abs(self.x * self.x + self.y * self.y + self.z * self.z - 1.0) > f32::EPSILON {
            false
        }
        else {
            true
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other : Self) -> Self {
        if self.w == 1.0 && other.w == 1.0 {
            panic!("adding two points wtf!!")
        }
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other : Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z,
            w: 0.0
        }
    }

}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w
        }
    }
}

// public static boolean nearlyEqual(float a, float b, float epsilon) {
//     final float absA = Math.abs(a);
//     final float absB = Math.abs(b);
//     final float diff = Math.abs(a - b);

//     if (a == b) { // shortcut, handles infinities
//         return true;
//     } else if (a == 0 || b == 0 || diff < Float.MIN_NORMAL) {
//         // a or b is zero or both are extremely close to it
//         // relative error is less meaningful here
//         return diff < (epsilon * Float.MIN_NORMAL);
//     } else { // use relative error
//         return diff / (absA + absB) < epsilon;
//     }
// }

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        f32::abs(self.x) - f32::abs(other.x) < f32::EPSILON &&
        f32::abs(self.y) - f32::abs(other.y) < f32::EPSILON &&
        f32::abs(self.z) - f32::abs(other.z) < f32::EPSILON &&
        f32::abs(self.w) - f32::abs(other.w) < f32::EPSILON
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.w == 1.0 {
            write!(f, "({0:.2}, {1:.2}, {2:.2})", self.x, self.y, self.z)
        }
        else {
            write!(f, "<{0:.2}, {1:.2}, {2:.2}>", self.x, self.y, self.z)
        }
    }
}

#[test]
fn add_vector() {
    let v1 = Vector::new(1.0,1.0,1.0, 0.0);
    let v2 = Vector::new(1.0,1.0,1.0, 0.0);
    let v3 = v1 + v2;
    assert_eq!(v3.x, 2.0);
    assert_eq!(v3.y, 2.0);
    assert_eq!(v3.z, 2.0);
    assert_eq!(v3.w, 0.0);
}
#[test]
fn sub_vector() {
    let v1 = Vector::new(3.0, 3.0, 3.0, 0.0);
    let v2 = Vector::new(1.0, 1.0, 1.0, 0.0);
    let v3 = v1 - v2;
    assert_eq!(v3.x, 2.0);
    assert_eq!(v3.y, 2.0);
    assert_eq!(v3.z, 2.0);
    assert_eq!(v3.w, 0.0);
}

#[test]
fn mul_vector() {
    let v1 = Vector::new(1.0, 1.0, 1.0, 0.0);
    let scalar = 2.0;
    let v2 = v1 * scalar;
    assert_eq!(v2.x, 2.0);
    assert_eq!(v2.y, 2.0);
    assert_eq!(v2.z, 2.0);
    assert_eq!(v2.w, 0.0);
}

#[test]
fn neg_vector() {
    let v1 = Vector::new(1.0, 1.0, 1.0, 0.0);
    let v2 = -v1;
    assert_eq!(v2.x, -1.0);
    assert_eq!(v2.y, -1.0);
    assert_eq!(v2.z, -1.0);
    assert_eq!(v2.w, 0.0);
}

#[test]
fn dot_product() {
    let v1 = Vector::new(1.0, 2.0, 3.0, 0.0);
    let v2 = Vector::new(4.0, 5.0, 6.0, 0.0);
    let dot = v1.dot(&v2);
    assert_eq!(dot, 32.0);
}

#[test]
fn cross_product() {
    let v1 = Vector::new(1.0, 0.0, 0.0, 0.0);
    let v2 = Vector::new(0.0, 1.0, 0.0, 0.0);
    let v3 = v1.cross(&v2);
    assert_eq!(v3.x, 0.0);
    assert_eq!(v3.y, 0.0);
    assert_eq!(v3.z, 1.0);
    assert_eq!(v3.w, 0.0);
}

#[test]
fn distance_between_vectors() {
    let v1 = Vector::new(1.0, 2.0, 3.0, 1.0);
    let v2 = Vector::new(4.0, 5.0, 6.0, 1.0);
    let distance = v1.distance(&v2);
    assert_eq!(distance, f32::sqrt(27.0));
}

#[test]
fn normalize_vector() {
    let mut v1 = Vector::new(3.0, 0.0, 4.0, 0.0);
    v1.normalize();
    assert!((v1.x - 0.6).abs() < f32::EPSILON);
    assert!((v1.y - 0.0).abs() < f32::EPSILON);
    assert!((v1.z - 0.8).abs() < f32::EPSILON);
    assert_eq!(v1.w, 0.0);
}

#[test]
fn equality() {
    let v1 = Vector::new(1.0,1.0,1.0, 0.0);
    let v2 = Vector::new(1.0,1.0,1.0, 0.0);
    let equal = v1 == v2;
    assert!(equal)
}