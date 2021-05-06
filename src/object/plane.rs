extern crate nalgebra as na;

use na::*;
use na::geometry::Point3;

use crate::types::*;
use super::{Surface, bound::*};

pub struct Plane {
    pub center: Point3f,        // Plane origin (used for texture mapping).
    pub normal: Unit3f, // Precomputed plane normal.

    x_axis: Vector3f, // Plane x-axis (The 3D direction that corresponds to the x-direction on the plane).
    y_axis: Vector3f, // Plane y-axis (The 3D direction that corresponds to the y-direction on the plane).

    texture: Box<dyn Fn(f32, f32) -> Texture> // Texture map.
                                              // Input coordinates are defined in terms of the axes above.
}

#[allow(dead_code)]
impl Plane {
    // Creates a new plane.
    pub fn new<F: 'static>(center: Point3f, x_axis: Vector3f, y_axis: Vector3f, texture: F) -> Self
        where F: Fn(f32, f32) -> Texture
    {
        Plane {
            center,
            normal: Unit::new_normalize(x_axis.cross(&y_axis)),
            x_axis: x_axis,
            y_axis: y_axis,
            texture: Box::new(texture)
        }
    }

    // Creates a new plane with the normal flipped.
    pub fn new_flip<F: 'static>(center: Point3f, x_axis: Vector3f, y_axis: Vector3f, texture: F) -> Self
        where F: Fn(f32, f32) -> Texture
    {
        Plane {
            center: center,
            normal: Unit::new_normalize(y_axis.cross(&x_axis)),
            x_axis: x_axis,
            y_axis: y_axis,
            texture: Box::new(texture)
        }
    }

    // Creates a new plane of a solid color.
    pub fn new_solid(center: Point3f, x_axis: Vector3f, y_axis: Vector3f, texture: Texture) -> Self
        { Plane::new(center, x_axis, y_axis, move |_, _| texture) }

    // Creates a new flipped plane of a solid color.
    pub fn new_solid_flip(center: Point3f, x_axis: Vector3f, y_axis: Vector3f, texture: Texture) -> Self
        { Plane::new_flip(center, x_axis, y_axis, move |_, _| texture) }


    // Creates a new XY-plane with the given texture map.
    pub fn xy(texture: impl 'static + Fn(f32, f32) -> Texture) -> Self
        { Plane::new(Point3::origin(), Vector3::x(), Vector3::y(), texture) }

    // Creates a new XZ-plane with the given texture map.
    pub fn xz(texture: impl 'static + Fn(f32, f32) -> Texture) -> Self
        { Plane::new(Point3::origin(), Vector3::x(), Vector3::z(), texture) }
}

impl Surface for Plane {
    fn intersect(&self, ray: Ray) -> Option<f32> {

        let d = self.normal.dot(&ray.direction);
        if d > -1e-3 { return None; }

        let t = (self.center - ray.origin).dot(&*self.normal) / d;

        if t >= 0.0 { Some(t) }
        else { None }
    }

    fn normal(&self, _point: Point3f) -> Unit3f { self.normal }

    fn get_texture(&self, point: Point3f) -> Texture {
        let rel_pos = point - self.center;
        let proj_point3 = rel_pos - (*self.normal * self.normal.dot(&rel_pos));

        let x = proj_point3.dot(&self.x_axis);
        let y = proj_point3.dot(&self.y_axis);

        (*self.texture)(x, y)
    }

    // Planes are infinite, so no finite
    // bounding sphere could possibly contain one.
    fn bound(&self) -> Bound { Bound::bypass() }
}
