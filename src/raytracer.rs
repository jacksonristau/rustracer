use std::sync::Arc;

use crate::math::ray::Ray;
use crate::graphics::color::Color;
use crate::math::vector::Vector;
use crate::scene::Scene;

use rayon::prelude::*;

pub struct Raytracer {
    pub scene: Scene,
    pub u: Vector,
    pub v: Vector,
    pub ul: Vector,
    pub ur: Vector,
    pub ll: Vector,
    pub lr: Vector,
    pub dh: Vector,
    pub dv: Vector,
    pub width: f64,
    pub height: f64,
}

impl Raytracer {
    pub fn new(scene: Scene) -> Self{
        if scene.view_dir.dot(&scene.up_dir) < -0.9 || scene.view_dir.dot(&scene.up_dir) > 0.9 {
            panic!("View direction and up direction are too close to parallel");
        }
        let mut u = scene.view_dir.cross(&scene.up_dir);
        u.normalize();
        let v = u.cross(&scene.view_dir);

        let aspect_ratio = scene.resolution.0 as f64 / scene.resolution.1 as f64;
        let d = 1.0;

        let width = if scene.parallel {
            scene.frustum_width
        } else {
            2.0 * d * (scene.hfov.to_radians() / 2.0).tan()
        };
        let height = width / aspect_ratio;

        let ul = (scene.eye_pos + scene.view_dir * d) - (u * (width / 2.0)) + (v * (height / 2.0));
        let ur = (scene.eye_pos + scene.view_dir * d) + (u * (width / 2.0)) + (v * (height / 2.0));
        let ll = (scene.eye_pos + scene.view_dir * d) - (u * (width / 2.0)) - (v * (height / 2.0));
        let lr = (scene.eye_pos + scene.view_dir * d) + (u * (width / 2.0)) - (v * (height / 2.0));

        let dh = (ur - ul) * (1.0 / (scene.resolution.0 as f64 - 1.0));
        let dv = (ll - ul) * (1.0 / (scene.resolution.1 as f64 - 1.0));

        Self {
            scene: scene,
            u: u,
            v: v,
            ul: ul,
            ur: ur,
            ll: ll,
            lr: lr,
            dh: dh,
            dv: dv,
            width: width,
            height: height
        }
    }

    pub fn trace(&self, ray : Ray) -> Color {
        let mut output = self.scene.bkg_color;
        let mut min_t = f64::INFINITY;
        for sphere in &self.scene.spheres {
            let t = ray.intersect_sphere(&sphere);
            if t > 0.0 {
                if t < min_t {
                    min_t = t;
                    output = self.shade( sphere.material_index);
                }
            }
        }
        output
    }

    pub fn shade(&self, m: usize) -> Color {
        let material = self.scene.materials[m];
        material.diffuse
    }

    pub fn trace_rays(self: Arc<Self>) -> Vec<Color>{
        println!("tracing rays...");
        let is_parallel = self.scene.parallel;
        let px_width = self.scene.resolution.0;
        let px_height = self.scene.resolution.1;

        let pixel_map: Vec<Color> = (0..px_height)
            .into_par_iter()
            .flat_map(|i| {
                let me = Arc::clone(&self);
                (0..px_width).into_par_iter().map(move |j| {
                    // this may need to be used
                    let _index = (i * px_width + j) as usize;
                    let color = if is_parallel {
                        me.trace(Ray::new(me.ul + (me.dh * j as f64) + (me.dv * i as f64), me.scene.view_dir))
                    } else {
                        me.trace(Ray::new(me.scene.eye_pos, me.ul + (me.dh * j as f64) + (me.dv * i as f64) - me.scene.eye_pos))
                    };
                    color
                })
            })
            .collect();
        println!("tracing complete.");
        pixel_map
    }
}