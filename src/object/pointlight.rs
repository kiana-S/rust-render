extern crate nalgebra as na;

use na::*;
use na::geometry::Point3;

use crate::types::*;
use super::*;

pub struct PointLight {
    pub pos: Point3<f32>,
    pub color: Color
}

impl PointLight {
    pub fn new(pos: Point3<f32>, color: Color) -> PointLight {
        PointLight {
            pos: pos,
            color: color
        }
    }

    fn check_point(&self, point: Point3<f32>, objects: &Vec<Object>) -> bool {
        let max_d = distance(&self.pos, &point);
        objects.iter()
               .filter_map(|obj| obj.intersect(Ray::from_points(self.pos, point)))
               .all(|d| d > max_d)
    }
}

impl Light for PointLight {
    fn illuminate(&self, point: Point3<f32>, objects: &Vec<Object>) -> Option<Color> {
        if self.check_point(point, objects) {
            Some(self.color)
        } else { None }
    }
}
