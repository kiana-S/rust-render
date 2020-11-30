extern crate nalgebra as na;

use na::*;
use na::geometry::Point3;

use crate::types::*;

pub trait Surface {
    fn intersect(&self, ray: Ray) -> Option<f32>;

    fn normal(&self, point: Point3<f32>) -> Unit<Vector3<f32>>;

    fn getcolor(&self, point: Point3<f32>) -> Color;
}
