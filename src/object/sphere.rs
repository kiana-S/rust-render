extern crate nalgebra as na;

use std::f32::consts::PI;

use na::*;
use na::geometry::Point3;

use crate::types::*;
use super::{Surface, bound::*};

pub struct Sphere {
    pub center: Point3f, // Center point of the sphere.
    pub radius: f32,         // Radius of the sphere.

    texture: Box<dyn Fn(f32, f32) -> Texture> // Texture map.
                                              // Uses spherical coordinates (normalized from 0-1) as input.
}

#[allow(dead_code)]
impl Sphere {
    // Creates a new sphere.
    pub fn new<F: 'static>(x: f32, y: f32, z: f32, radius: f32, texture: F) -> Self
        where F: Fn(f32, f32) -> Texture
    {
        Sphere {
            center: Point3::new(x, y, z),
            radius: radius,
            texture: Box::new(texture)
        }
    }

    // Creates a new sphere of a solid color.
    pub fn new_solid(x: f32, y: f32, z: f32, radius: f32, texture: Texture) -> Self
        { Sphere::new(x, y, z, radius, move |_, _| texture) }
}

impl Surface for Sphere {
    fn intersect(&self, ray: Ray) -> Option<f32> {
        fn solve_quadratic(b: f32, c: f32) -> Option<(f32, f32)> {
            let discr = b * b - 4.0 * c;

            if discr < 0.0 { None }
            else if discr == 0.0 {
                let x = -0.5 * b;
                Some((x, x))
            } else {
                let q = if b > 0.0 { -0.5 * (b + discr.sqrt()) } else { -0.5 * (b - discr.sqrt()) };
                Some((q, c / q))
            }
        }

        let l = ray.origin - self.center;
        let b = 2.0 * ray.direction.dot(&l);
        let c = l.norm_squared() - self.radius * self.radius;

        let (mut t0, mut t1) = solve_quadratic(b, c)?;

        if t0 > t1 { std::mem::swap(&mut t0, &mut t1); }

        if t0 >= 0.0 { Some(t0) }
        else if t1 >= 0.0 { Some(t1) }
        else { None }
    }

    fn normal(&self, point: Point3f) -> Unit3f {
        Unit::new_normalize(point - self.center)
    }

    fn gettexture(&self, point: Point3f) -> Texture {
        let normal = self.normal(point);

        // In this particular case, the normal is simular to a point on a unit sphere
        // centred around the origin. We can thus use the normal coordinates to compute
        // the spherical coordinates of the point.
        let x = 0.5 + normal.z.atan2(normal.x) / (2.0 * PI);
        let y = normal.y.acos() / PI;

        (*self.texture)(x, y)
    }

    fn bound(&self) -> Bound { Bound::bypass() }
}
