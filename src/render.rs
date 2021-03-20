extern crate nalgebra as na;

use std::f32::consts::PI;
use std::cmp::Ordering;

use na::*;
use na::geometry::Point3;

use crate::types::*;
use crate::object::*;

fn trace(ray: Ray, objects: &Vec<Object>) -> Option<(&Object, f32)> {
    objects.iter()
           .filter_map(|&obj| obj.intersect(ray)
                                .map(|x| (obj, x)))
           .min_by(|&a, &b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
}

fn light_point(objects: &Vec<Object>, obj: &Object, point: Point3f, light: &dyn Light) -> Color {
    if light.check_shadow(point, objects) {

        let texture = obj.gettexture(point);

        light.getcolor(point) * (texture.albedo / PI) * light.intensity(point) * obj.normal(point).dot(&*light.direction(point))
    } else {
        // Point is in shadow
        Color::black()
    }
}

pub fn cast_ray(ray: Ray, scene: &Scene) -> Color {
    if let Some((obj, dist)) = trace(ray, &scene.objects) {
        let point = ray.project(dist);
        let surface_color = obj.gettexture(point).color;

        scene.lights.iter()
                    .map(|&light| light_point(&scene.objects, obj, point, &*light))
                    .fold(Color::black(), |acc, c| acc + c) * surface_color
    }
    else { scene.background }
}
