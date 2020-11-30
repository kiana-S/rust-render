extern crate nalgebra as na;

use na::*;
use na::geometry::Point3;

use crate::types::*;

pub struct Plane {
    pub center: Point3<f32>,
    pub normal: Unit<Vector3<f32>>,

    pub texture: Box<dyn Fn(f32, f32) -> Color>
}

impl Plane {
    pub fn new<F: 'static>(center: Point3<f32>, normal: Vector3<f32>, texture: F) -> Self
        where F: Fn(f32, f32) -> Color
    {
        Plane {
            center: center,
            normal: Unit::new_normalize(normal),
            texture: Box::new(texture)
        }
    }

    pub fn new_solid(center: Point3<f32>, normal: Vector3<f32>, color: Color) -> Self
        { Plane::new(center, normal, move |_, _| color) }

    pub fn intersect(&self, ray: Ray) -> Option<f32> {

        let d = self.normal.dot(&ray.direction);
        if d < 1e-6 { return None; }

        let t = (self.center - ray.origin).dot(&*self.normal) / d;

        if t >= 0.0 { Some(t) }
        else { None }
    }

    pub fn getcolor(&self, point: Point3<f32>) -> Color {
        unimplemented!()
    }

    pub fn normal(&self, point: Point3<f32>) -> Unit<Vector3<f32>> { self.normal }
}
