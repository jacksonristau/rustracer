use super::triangle::Triangle;
use super::vector::Vector;
use super::sphere::Sphere;
use std::f32;
use std::fmt;
use std::cmp::PartialEq;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub o: Vector,
    pub d: Vector,
}

impl Ray {
    pub fn new(o: Vector, mut d: Vector) -> Self {
        d.normalize();
        Ray { o: o, d: d }
    }

    pub fn get_point(&self, t: f32) -> Vector{
        (self.d * t) + self.o
    }

    pub fn intersect_sphere(&self, sphere : &Sphere) -> f32 {
        let b = 2.0 * 
            (self.d.x * (self.o.x - sphere.center.x) +
            self.d.y * (self.o.y - sphere.center.y) +
            self.d.z * (self.o.z - sphere.center.z));
        let c = 
            f32::powf(self.o.x - sphere.center.x, 2.0) +
            f32::powf(self.o.y - sphere.center.y, 2.0) +
            f32::powf(self.o.z - sphere.center.z, 2.0) -
            f32::powf(sphere.radius, 2.0);
        
        let discrim = f32::powf(b, 2.0) - (4.0 * c);
        if discrim < 0.0 {
            -1.0
        }
        else if discrim == 0.0 {
            -b / 2.0
        }
        else {
            let t1 = (-b + f32::sqrt(discrim)) / 2.0;
            let t2 = (-b - f32::sqrt(discrim)) / 2.0;
            
            if t1 < f32::EPSILON && t2 > f32::EPSILON {
                t2
            }
            else if t1 > f32::EPSILON && t2 < f32::EPSILON {
                t1
            }
            else if t1 < f32::EPSILON && t2 <= f32::EPSILON {
                -1.0
            }
            else {
                f32::min(t1, t2)
            }
        }
    }

    pub fn intersect_plane(&self, normal: &Vector, point: &Vector) -> f32 {
        let denom = normal.dot(&self.d);
        if denom > -f32::EPSILON && denom < f32::EPSILON {
            -1.0
        }
        else {
            let d = -normal.dot(point);
            -(normal.dot(&self.o) + d) / denom
        }
    }

    pub fn intersect_triangle(&self, triangle : &Triangle, coords: Option<&mut [f32; 3]>) -> f32 {
        let e1 = triangle.position.1 - triangle.position.0;
        let e2 = triangle.position.2 - triangle.position.0;

        let n = e1.cross(&e2);
        let t = self.intersect_plane(&n, &triangle.position.0);
        if t < 0.0 {
            -1.0
        }
        else {
            let d11 = e1.dot(&e1);
            let d12 = e1.dot(&e2);
            let d22 = e2.dot(&e2);
            let det = (d11 * d22) - (d12 * d12);
            if det > -f32::EPSILON && det < f32::EPSILON {
                -1.0
            }
            else {
                let p = self.get_point(t);
                let ep = p - triangle.position.0;
                let dp1 = ep.dot(&e1);
                let dp2 = ep.dot(&e2);

                let beta = ((d22 * dp1) - (d12 * dp2)) / det;
                let gamma = ((d11 * dp2) - (d12 * dp1)) / det;
                let alpha = 1.0 - (beta + gamma);
                if 0.0 <= alpha && alpha <= 1.0 && 0.0 <= beta && beta <= 1.0 && 0.0 <= gamma && gamma <= 1.0 {
                    match coords {
                        None => t,
                        Some(ptr) => {
                            ptr[0] = alpha;
                            ptr[1] = beta;
                            ptr[2] = gamma;
                            t
                        }
                    }
                }
                else {
                    -1.0
                }
            }
        }
    }

    pub fn reflect(&self, n: &Vector) -> Vector {
        let i = -self.d;
        let mut r = (*n * 2.0 * n.dot(&i)) - i;
        r.normalize();
        return r;
    }

    pub fn refract(&self, n: &Vector, n1: f32, n2: f32) -> Vector {
        let i = -self.d;
        let snell = n1 / n2;
        let ndoti = n.dot(&i);
        let b = ((*n * ndoti) - i) * snell;
        let discrim = 1.0 - f32::powf(snell, 2.0) * (1.0 - f32::powf(ndoti, 2.0));

        if discrim < 0.0 {
            return Vector::new(0.0,0.0,0.0,0.0)
        }
        else {
            let a = -*n * f32::sqrt(discrim);
            a + b
        }
    }
}

impl PartialEq for Ray {
    fn eq(&self, other: &Self) -> bool {
        self.o == other.o &&
        self.d == other.d
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "origin: {}, direction: {}", self.o, self.d)
    }
}

#[test]
fn trace_point() {
    let origin = Vector::new(0.0,0.0,0.0,1.0);
    let direction = Vector::new(0.0,1.0,0.0,0.0);
    let ray = Ray::new(origin, direction);
    let t = 5.0;
    assert_eq!(ray.get_point(t), Vector::new(0.0, 5.0, 0.0, 1.0));
}
#[test]
fn test_ray_new() {
    let origin = Vector::new(1.0, 2.0, 3.0, 1.0);
    let direction = Vector::new(4.0, 5.0, 6.0, 0.0);
    let ray = Ray::new(origin, direction);
    assert_eq!(ray.o, origin);
    assert!(ray.d.is_normalized());
}

#[test]
fn test_ray_get_point() {
    let origin = Vector::new(0.0, 0.0, 0.0, 1.0);
    let direction = Vector::new(0.0, 1.0, 0.0, 0.0);
    let ray = Ray::new(origin, direction);
    let t = 5.0;
    assert_eq!(ray.get_point(t), Vector::new(0.0, 5.0, 0.0, 1.0));
}

#[test]
fn test_ray_intersect_sphere() {
    let origin = Vector::new(0.0, 0.0, -5.0, 1.0);
    let direction = Vector::new(0.0, 0.0, 1.0, 0.0);
    let ray = Ray::new(origin, direction);
    let sphere = Sphere::new(Vector::new(0.0, 0.0, 0.0, 1.0), 1.0, 0);
    let t = ray.intersect_sphere(&sphere);
    assert!(t > 0.0);
}

#[test]
fn test_ray_intersect_plane() {
    let origin = Vector::new(0.0, 0.0, 0.0, 1.0);
    let direction = Vector::new(0.0, -1.0, 0.0, 0.0);
    let ray = Ray::new(origin, direction);
    let normal = Vector::new(0.0, 1.0, 0.0, 0.0);
    let point = Vector::new(0.0, -5.0, 0.0, 1.0);
    let t = ray.intersect_plane(&normal, &point);
    assert!(t > 0.0);
}

#[test]
fn test_ray_intersect_triangle() {
    let origin = Vector::new(0.0, 0.0, 0.0, 1.0);
    let direction = Vector::new(0.0, 0.0, 1.0, 0.0);
    let ray = Ray::new(origin, direction);
    let vertices = (
        Vector::new(0.0, 1.0, 5.0, 1.0),
        Vector::new(-1.0, -1.0, 5.0, 1.0),
        Vector::new(1.0, -1.0, 5.0, 1.0),
    );
    let triangle = Triangle::new(vertices.0, vertices.1, vertices.2, Vector::new(0.0, 0.0, 1.0, 0.0), Vector::new(0.0, 0.0, 1.0, 0.0), Vector::new(0.0, 0.0, 1.0, 0.0), [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0,0.0,0.0]);
    let mut coords = [0.0; 3];
    let t = ray.intersect_triangle(&triangle, Some(&mut coords));
    assert!(t > 0.0);
    assert!(coords.iter().all(|&c| c >= 0.0 && c <= 1.0));
}

#[test]
fn test_ray_reflect() {
    let origin = Vector::new(0.0, 0.0, 0.0, 1.0);
    let direction = Vector::new(1.0, -1.0, 0.0, 0.0);
    let ray = Ray::new(origin, direction);
    let normal = Vector::new(0.0, 1.0, 0.0, 0.0);
    let reflected = ray.reflect(&normal);
    let mut expected = Vector::new(1.0, 1.0, 0.0, 0.0);
    expected.normalize();
    assert_eq!(reflected, expected);
}

#[test]
fn test_ray_refract() {
    let origin = Vector::new(0.0, 0.0, 0.0, 1.0);
    let direction = Vector::new(1.0, -1.0, 0.0, 0.0);
    let ray = Ray::new(origin, direction);
    let normal = Vector::new(0.0, 1.0, 0.0, 0.0);
    let refracted = ray.refract(&normal, 1.0, 1.5);
    println!("{}", refracted);
    assert!(refracted.is_normalized());
}
