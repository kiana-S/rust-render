
mod sphere; pub use sphere::*;
mod plane; pub use plane::*;
mod triangle; pub use triangle::*;

use na::*;

use crate::types::*;

pub trait Surface {
    fn intersect(&self, ray: Ray) -> Option<f32>;

    fn normal(&self, point: Point3<f32>) -> Unit<Vector3<f32>>;

    fn getcolor(&self, point: Point3<f32>) -> Color;
}

pub struct Object {
    pub surface: Box<dyn Surface>
}

impl Object {
    pub fn new<S: 'static + Surface>(surface: S) -> Self {
        Object { surface: Box::new(surface) }
    }

    pub fn intersect(&self, ray: Ray) -> Option<f32> { self.surface.intersect(ray) }
    pub fn normal(&self, point: Point3<f32>) -> Unit<Vector3<f32>> { self.surface.normal(point) }
    pub fn getcolor(&self, point: Point3<f32>) -> Color { self.surface.getcolor(point) }
}

pub type Scene = Vec<Object>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn obj_getcolor() {
        let sphere = Object::new(Sphere::new_solid(0.0, 0.0, 0.0, 1.0, Color::white()));

        let point = Point3::new(1.0, 0.0, 0.0);

        assert_eq!(sphere.getcolor(point), Color::white());
    }
}
