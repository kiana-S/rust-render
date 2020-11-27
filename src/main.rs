extern crate nalgebra as na;

use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;

use na::*;
use na::geometry::Point3;

mod camera; use camera::*;
mod types; use types::*;
mod object; use object::*;

fn cast_ray(ray: Ray, scene: &Scene) -> Color {
    //Color::new(0.0, -ray.direction.x, ray.direction.y)

    let closest = scene.iter()
                       .filter_map(|obj| obj.intersect(ray)
                                            .map(|x| (obj, x)))
                       .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));

    if closest.is_some() {
        Color::new(1.0, 1.0, 1.0)
    } else {
        Color::new(0.0, 0.0, 0.0)
    }
}

fn render(camera: &Camera, scene: &Scene, filename: &str) -> std::io::Result<()> {
    let width  = camera.image_size.x;
    let height = camera.image_size.y;

    let mut buffer: Vec<Color> = Vec::with_capacity((width * height) as usize);

    for j in 0..height {
        for i in 0..width {
            let ray = camera.raycast(i, j);
            buffer.push(cast_ray(ray, &scene));
        }
        // println!("Rendered row {}", j);
    }

    let mut file = File::create(filename)?;
    file.set_len(0)?;
    file.write_all(format!("P6\n{} {}\n255\n", width, height).as_bytes())?;
    for color in buffer.into_iter() {
        file.write_all(&color.to_byte_array())?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {

    let camera = Camera::new(Point3::new(0.0,0.0,0.0), Vector3::new(0.0,0.0,-1.0), 1.0, 2.0, 2.0, 400, 400);

    let scene = vec![
        Object::Sphere(Sphere::new(0.0,0.0,-5.0,2.0)),
        Object::Sphere(Sphere::new(-3.0,0.0,-8.0,2.5))
    ];

    render(&camera, &scene, "out.ppm")
}
