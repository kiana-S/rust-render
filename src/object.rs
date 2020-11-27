
mod sphere; pub use sphere::*;
mod triangle; pub use triangle::*;

use na::*;

use crate::types::Ray;

pub enum Object {
    Sphere(Sphere)
}

impl Object {
    pub fn intersect(&self, ray: Ray) -> Option<f32> {
        match *self {
            Object::Sphere(ref sphere) => sphere.intersect(ray)
        }
    }

    pub fn normal(&self, ray: Ray) -> Unit<Vector3<f32>> {
        match *self {
            Object::Sphere(ref sphere) => sphere.normal(ray)
        }
    }
}

pub type Scene = Vec<Object>;
