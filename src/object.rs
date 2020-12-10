
mod sphere; pub use sphere::*;
mod plane; pub use plane::*;
mod triangle; pub use triangle::*;
mod bound; pub use bound::*;

use na::*;

use crate::types::*;

// A trait for types that can be in Objects.
pub trait Surface {

    // Takes in a ray and performs an intersection test
    // on itself. If the ray intersects the object,
    // returns the distance to the intersection point.
    fn intersect(&self, ray: Ray) -> Option<f32>;

    // Takes in a point (assumed to be on the object's surface)
    // and returns the normal vector off of that point.
    fn normal(&self, point: Point3<f32>) -> Unit<Vector3<f32>>;

    // Takes in a point (assumed to be on the object's surface)
    // and returns the color information on that point.
    fn getcolor(&self, point: Point3<f32>) -> Color;

    // Creates a bounding sphere around the object.
    fn bound(&self) -> Bound;
}

pub struct Object {
    pub surface: Box<dyn Surface>,
    bound: Bound
}

#[allow(dead_code)]
impl Object {
    // Creates a new object with a custom bounding sphere.
    pub fn new_(surface: impl 'static + Surface, center: Point3<f32>, radius: f32) -> Self {
        Object {
            surface: Box::new(surface),
            bound: Bound { center: center, radius: radius, bypass: false }
        }
    }

    // Creates a new object with no bounding sphere.
    pub fn new_boundless(surface: impl 'static + Surface) -> Self {
        Object {
            surface: Box::new(surface),
            bound: Bound::bypass()
        }
    }

    // Creates a new object with the default bounding sphere.
    pub fn new(surface: impl 'static + Surface) -> Self {
        let bound = surface.bound();
        Object {
            surface: Box::new(surface),
            bound: bound
        }
    }


    pub fn intersect(&self, ray: Ray) -> Option<f32> {
        if self.bound.is_intersected(ray) {
            self.surface.intersect(ray)
        } else { None }
    }
    pub fn normal(&self, point: Point3<f32>) -> Unit<Vector3<f32>> { self.surface.normal(point) }
    pub fn getcolor(&self, point: Point3<f32>) -> Color { self.surface.getcolor(point) }
}

pub struct Scene {
    pub objects: Vec<Object>,

    pub background: Color
}

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
