extern crate nalgebra as na;

use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;

use na::*;
use na::geometry::Point3;

mod camera; use camera::*;
mod types; use types::*;
mod object; use object::*;

fn trace(ray: Ray, objects: &Vec<Object>) -> Option<(&Object, f32)> {
    objects.iter()
           .filter_map(|obj| obj.intersect(ray)
                                .map(|x| (obj, x)))
           .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
}

fn cast_ray(ray: Ray, scene: &Scene) -> Color {
    if let Some((obj, dist)) = trace(ray, &scene.objects) {
        let point = ray.project(dist);

        obj.getcolor(point)
    }
    else { scene.background }
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

    let camera = Camera::new(Point3::new(0.0,0.0,0.0), Vector3::new(0.0,0.0,1.0), 1.0, 16.0 / 9.0, 2.0, 480);

    let scene = Scene {
        objects: vec![
            Object::new(TriangleMesh::singleton(Point3::new(-1.0, -1.0, 2.0), Point3::new(0.0, 1.0, 2.0), Point3::new(1.0, -1.0, 2.0), |t, u, v| Color::new(t, u, v)))
        ],
        background: Color::black()
    };

    render(&camera, &scene, "out.ppm")
}
