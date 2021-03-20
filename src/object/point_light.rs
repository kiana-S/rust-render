extern crate nalgebra as na;

use na::*;

use crate::types::*;
use super::*;

pub struct PointLight {
    pub pos: Point3f,
    pub color: Color,
    pub intensity: f32
}

#[allow(dead_code)]
impl PointLight {
    pub fn new(pos: Point3f, color: Color, intensity: f32) -> PointLight {
        PointLight { pos, color, intensity }
    }
}

impl Light for PointLight {
    fn check_shadow(&self, point: Point3f, objects: &Vec<Object>) -> bool {
        let max_d = distance(&self.pos, &point);
        objects.iter()
               .filter_map(|obj| obj.intersect(Ray::from_points(self.pos, point)))
               .all(|d| d - max_d > -1e-3 )
    }

    fn get_color(&self, _point: Point3f) -> Color { self.color }

    fn intensity(&self, _point: Point3f) -> f32 { self.intensity }

    fn direction(&self, point: Point3f) -> Unit3f {
        Unit::new_normalize(self.pos - point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_light_check_shadow() {
        let light = PointLight::new(Point3::new(0.0, 1.0, 0.0), Color::white(), 1.0);
        let block = Object::new(Sphere::new_solid(0.0, 0.5, 0.0, 0.1, Texture::new(0.0, 0.0, 0.0, 0.0)));

        assert!(light.check_shadow(Point3::origin(), &Vec::new()));
        assert!(!light.check_shadow(Point3::origin(), &vec![block]));
    }
}
