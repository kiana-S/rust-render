extern crate nalgebra as na;

use na::distance;
use na::geometry::Point3;

use crate::types::Ray;

// A bounding sphere, used for
// intersection test optimization.
#[derive(Debug)]
pub struct Bound {
    pub center: Point3<f32>,
    pub radius: f32,

    // If true, then the bounding sphere is disabled.
    pub bypass: bool
}

impl Bound {
    pub fn is_intersected(&self, ray: Ray) -> bool {
        if self.bypass { return true; }

        let l = ray.origin - self.center;
        l.norm_squared() >= self.radius * self.radius
    }

    pub fn contains(&self, point: &Point3<f32>) -> bool { distance(&self.center, point) < self.radius }

    pub fn bypass() -> Self { Bound { center: Point3::origin(), radius: 0.0, bypass: true } }
}
