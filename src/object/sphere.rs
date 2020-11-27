extern crate nalgebra as na;

use na::*;
use na::geometry::Point3;

use crate::types::*;

pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32
}

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, radius: f32) -> Self {
        Sphere {
            center: Point3::new(x, y, z),
            radius: radius
        }
    }

    pub fn intersect(&self, ray: Ray) -> Option<f32> {
        fn solve_quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
            let discr = b * b - 4.0 * a * c;

            if discr < 0.0 { None }
            else if discr == 0.0 {
                let x = -0.5 * b / a;
                Some((x, x))
            } else {
                let q = if b > 0.0 { -0.5 * (b + discr.sqrt()) } else { -0.5 * (b - discr.sqrt()) };
                Some((q / a, c / q))
            }
        }

        let l = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&l);
        let c = l.dot(&l) - self.radius * self.radius;

        let (mut t0, mut t1) = solve_quadratic(a, b, c)?;

        if t0 > t1 { std::mem::swap(&mut t0, &mut t1); }

        if t0 > 0.0 { Some(t0) }
        else if t1 > 0.0 { Some(t1) }
        else { None }
    }

    pub fn normal(&self, ray: Ray) -> Unit<Vector3<f32>> {
        unimplemented!()
    }
}
