extern crate nalgebra as na;

use std::fs::File;
use std::io::Write;

use na::*;
use na::geometry::Point3;

mod camera; use camera::*;
mod types; use types::*;
mod object; use object::*;
mod render; use render::*;

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
        lights: Vec::new(),
        background: Color::black()
    };

    render(&camera, &scene, "out.ppm")
}
