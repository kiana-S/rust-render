extern crate nalgebra as na;

use std::cmp::Ordering;

use na::*;
use na::geometry::Point3;

use crate::types::*;
use crate::object::*;

fn trace(ray: Ray, objects: &Vec<Object>) -> Option<(&Object, f32)> {
    objects.iter()
           .filter_map(|obj| obj.intersect(ray)
                                .map(|x| (obj, x)))
           .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
}

pub fn cast_ray(ray: Ray, scene: &Scene) -> Color {
    if let Some((obj, dist)) = trace(ray, &scene.objects) {
        let point = ray.project(dist);
        let surface_texture = obj.gettexture(point);
        surface_texture.color
    }
    else { scene.background }
}
