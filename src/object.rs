
mod sphere; pub use sphere::*;
mod triangle; pub use triangle::*;

use na::*;

use crate::types::*;

pub enum Object {
    Sphere(Sphere)
}

impl Object {
    pub fn intersect(&self, ray: Ray) -> Option<f32> {
        match *self {
            Object::Sphere(ref sphere) => sphere.intersect(ray)
        }
    }

    pub fn getcolor(&self, point: Point3<f32>) -> Color {
        match *self {
            Object::Sphere(ref sphere) => sphere.getcolor(point)
        }
    }

    pub fn normal(&self, point: Point3<f32>) -> Unit<Vector3<f32>> {
        match *self {
            Object::Sphere(ref sphere) => sphere.normal(point)
        }
    }
}

pub type Scene = Vec<Object>;
