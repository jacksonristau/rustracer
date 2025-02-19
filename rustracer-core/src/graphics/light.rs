use serde::{Deserialize, Serialize};

use crate::math::vector::Vector;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Light {
    pub v : Vector,
    pub attenuation : (f32, f32, f32),
    pub i : f32,
}

impl Light {
    pub fn new(v : Vector, attenuation : (f32, f32, f32), i : f32) -> Self {
        Light {
            v,
            attenuation,
            i
        }
    }

    pub fn attenuate(&self, distance : f32) -> f32 {
        self.i / (self.attenuation.0 + self.attenuation.1 * distance + self.attenuation.2 * distance * distance)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_new() {
        let v = Vector::new(1.0, 2.0, 3.0, 0.0);
        let attenuation = (1.0, 0.5, 0.2);
        let i = 10.0;
        let light = Light::new(v, attenuation, i);

        assert_eq!(light.v, v);
        assert_eq!(light.attenuation, attenuation);
        assert_eq!(light.i, i);
    }

    #[test]
    fn test_light_attenuate() {
        let v = Vector::new(1.0, 2.0, 3.0, 0.0);
        let attenuation = (1.0, 0.5, 0.2);
        let i = 10.0;
        let light = Light::new(v, attenuation, i);

        let distance = 2.0;
        let expected_attenuation = i / (1.0 + 0.5 * distance + 0.2 * distance * distance);
        assert_eq!(light.attenuate(distance), expected_attenuation);
    }

    #[test]
    fn test_light_attenuate_zero_distance() {
        let v = Vector::new(1.0, 2.0, 3.0, 0.0);
        let attenuation = (1.0, 0.5, 0.2);
        let i = 10.0;
        let light = Light::new(v, attenuation, i);

        let distance = 0.0;
        let expected_attenuation = i / 1.0;
        assert_eq!(light.attenuate(distance), expected_attenuation);
    }

    #[test]
    fn test_light_attenuate_large_distance() {
        let v = Vector::new(1.0, 2.0, 3.0, 0.0);
        let attenuation = (1.0, 0.5, 0.2);
        let i = 10.0;
        let light = Light::new(v, attenuation, i);

        let distance = 100.0;
        let expected_attenuation = i / (1.0 + 0.5 * distance + 0.2 * distance * distance);
        assert_eq!(light.attenuate(distance), expected_attenuation);
    }
}
