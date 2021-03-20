
mod sphere; pub use sphere::*;
mod plane; pub use plane::*;
mod triangle; pub use triangle::*;
mod bound; pub use bound::*;
mod point_light; pub use point_light::*;

use crate::types::*;

// A trait for types that can be in Objects.
pub trait Surface {

    // Takes in a ray and performs an intersection test
    // on itself. If the ray intersects the object,
    // returns the distance to the intersection point.
    fn intersect(&self, ray: Ray) -> Option<f32>;

    // Takes in a point (assumed to be on the object's surface)
    // and returns the normal vector off of that point.
    fn normal(&self, point: Point3f) -> Unit3f;

    // Takes in a point (assumed to be on the object's surface)
    // and returns the texture information on that point.
    fn get_texture(&self, point: Point3f) -> Texture;

    // Creates a bounding sphere around the object.
    fn bound(&self) -> Bound;
}

pub struct Object {
    pub surface: Box<dyn Surface>,
    bound: Bound
}

impl Object {
    // Creates a new object with the default bounding sphere.
    pub fn new(surface: impl 'static + Surface) -> Self {
        let bound = surface.bound();
        Object {
            surface: Box::new(surface),
            bound
        }
    }


    pub fn intersect(&self, ray: Ray) -> Option<f32> {
        if self.bound.is_intersected(ray) {
            self.surface.intersect(ray)
        } else { None }
    }
    pub fn normal(&self, point: Point3f) -> Unit3f { self.surface.normal(point) }
    pub fn get_texture(&self, point: Point3f) -> Texture { self.surface.get_texture(point) }
}

pub trait Light {
    // Determine if the light is able to illuminate the point.
    fn check_shadow(&self, point: Point3f, objects: &Vec<Object>) -> bool;

    // Compute color on a point.
    fn get_color(&self, point: Point3f) -> Color;

    // Compute intensity on a point.
    fn intensity(&self, point: Point3f) -> f32;

    // Return the direction from the point to the light source.
    fn direction(&self, point: Point3f) -> Unit3f;
}

pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Box<dyn Light>>,
    pub background: Color
}
