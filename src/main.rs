extern crate nalgebra as na;

use std::time::Instant;
use std::fs::File;
use std::io::Write;

use na::*;

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

    let camera = Camera::new(Point3::new(0.0,0.0,3.0), Vector3::new(0.0,0.0,-1.0), 1.0, 16.0 / 9.0, 2.0, 720);

    let scene = Scene {
        objects: vec![
            Object::new(Sphere::new_solid(1.0, 0.2, 1.7, 0.2, Texture::new(1.0, 1.0, 1.0, 1.0))),
            Object::new(Sphere::new_solid(0.0, -1.0, 0.0, 1.0, Texture::new(1.0, 1.0, 1.0, 1.0)))
        ],
        lights: vec![
            Box::new(PointLight::new(Point3::new(1.5, 1.0, 2.5), Color::white(), 3.0)),
            Box::new(PointLight::new(Point3::new(1.0, -0.4, 1.5), Color::white(), 2.0))
        ],
        background: Color::gray(0.5)
    };

    let before = Instant::now();

    render(&camera, &scene, "out.ppm")?;

    println!("{}", before.elapsed().as_millis());

    Ok(())
}
