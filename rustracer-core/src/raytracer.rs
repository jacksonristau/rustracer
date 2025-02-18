use std::f64::EPSILON;
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

    pub fn depth_cue(&self, i : Color, view_distance : f64) -> Color{
        let alpha_dc;
        if view_distance <= self.scene.dist.0 {
            alpha_dc = self.scene.alpha.1;
        }
        else if view_distance >= self.scene.dist.1 {
            alpha_dc = self.scene.alpha.0;
        }
        else {
            alpha_dc = self.scene.alpha.0 + (self.scene.alpha.1 - self.scene.alpha.0) * ((self.scene.dist.1 - view_distance) / (self.scene.dist.1 - self.scene.dist.0));
        }
        (i * alpha_dc) + (self.scene.dc * (1.0 -  alpha_dc))
    }

    pub fn trace(&self, ray : Ray) -> Color {
        let mut output = self.scene.bkg_color;
        let mut min_t = f64::INFINITY;
        let mut hit_index = 0;
        let mut hit = false;
        for (i, sphere) in self.scene.spheres.iter().enumerate() {
            let t = ray.intersect_sphere(&sphere);
            if t > EPSILON {
                if t < min_t {
                    min_t = t;
                    hit_index = i;
                    hit = true;
                }
            }
        }
        if hit {
            let material_index = self.scene.spheres[hit_index].material_index;
            let intersection = ray.get_point(min_t);
            let mut n = intersection - self.scene.spheres[hit_index].center;
            n.normalize();
            output = self.shade(material_index, intersection, n, ray);
        }
        output
    }

    pub fn shade(&self, m: usize, x_p : Vector, normal : Vector, i_ray : Ray) -> Color {
        let material = self.scene.materials[m];
        let mut final_color = material.diffuse * material.k_a;
        for light in &self.scene.lights {
            let is_point = light.v.w == 1.0;
            let mut s_flag = 1.0;

            let mut l = if is_point {
                light.v - x_p
            } else {
                -light.v
            };
            let d = x_p.distance(&light.v);
            l.normalize();
            let r =  Ray::new(x_p, l);

            for sphere in &self.scene.spheres {
                if sphere.material_index == m {
                    continue;
                }
                let t = r.intersect_sphere(sphere);
                let surface_alpha = material.alpha;
                let is_between = if is_point {
                    EPSILON < t && t < d
                }
                else {
                    t > EPSILON
                };
                if is_between {
                    s_flag = s_flag * (1.0 - surface_alpha);
                }
            }
            let mut i = i_ray.d * -1.0;
            let ndotl = normal.dot(&l);
            if ndotl < 0.0 { continue; }
            i.normalize();
            let mut h = l + i;
            h.normalize();
            let mut ndoth = normal.dot(&h);
            if ndoth < 0.0 { ndoth = 0.0; }
            let diffuse = material.diffuse * ndotl * material.k_d;
            let specular = material.specular * ndoth.powi(material.n_val) * material.k_s;
            final_color = final_color +  ((diffuse + specular) * light.i * s_flag);
        }
        final_color = self.depth_cue(final_color, self.scene.eye_pos.distance(&x_p));
        final_color
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