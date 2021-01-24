extern crate nalgebra as na;

use na::*;
use na::geometry::{Point2, Point3};

use crate::types::Ray;

#[derive(Debug)]
pub struct Camera {
    matrix: Isometry3<f32>, // The transformation that stores the
                            // position and orientation of the camera. (Not actually a matrix, but w/e)

    focal_length: f32,         // The distance from the camera origin to the canvas.
    canvas_size: Vector2<f32>, // The size of the canvas within the world space.

    pub image_size: Vector2<u32> // The size of the final image in pixels.
}

impl Camera {

    // Constructs a new camera from a position and viewing direction.
    pub fn new_(pos: Point3<f32>, dir: Vector3<f32>, up: Vector3<f32>,
            focal_length: f32, aspect_ratio: f32, canvas_y: f32, image_y: u32) -> Self {
        let iso = Isometry3::face_towards(&pos, &(pos + dir), &up);
        Camera {
            matrix: iso,
            focal_length: focal_length,
            canvas_size: Vector2::new(canvas_y  * aspect_ratio, canvas_y),
            image_size: Vector2::new((image_y as f32 * aspect_ratio) as u32, image_y)
        }
    }

    // Constructs a new camera from a position and viewing direction
    // (assuming the camera is oriented upright).
    pub fn new(pos: Point3<f32>, dir: Vector3<f32>,
            focal_length: f32, aspect_ratio: f32, canvas_y: f32, image_y: u32) -> Self
        { Camera::new_(pos, dir, Vector3::y(), focal_length, aspect_ratio, canvas_y, image_y) }

    pub fn pos(&self) -> Point3<f32> { Point3::from(self.matrix.translation.vector) }

    // Takes a 2D point in the image space and
    // maps it to the 3D point on the canvas.
    fn project(&self, x: u32, y: u32) -> Point3<f32> {
        // convert point from raster coordinates to center-based coordinates
        let pixelndc = Point2::new(x as f32 + 0.5 - self.image_size.x as f32 * 0.5, -(y as f32 + 0.5) + self.image_size.y as f32 * 0.5);

        let point: Point3<f32> = Point::from(pixelndc.coords.component_div(&self.image_size.map(|x| x as f32))
                                                            .component_mul(&self.canvas_size)
                                                            .fixed_resize(self.focal_length));
        self.matrix * point
    }

    // Takes a 2D point in the image space and
    // returns a ray in the world space, for use in raytracing.
    pub fn raycast(&self, x: u32, y: u32) -> Ray {
        Ray::from_points(self.pos(), self.project(x, y))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn round(point: Point3<f32>) -> Point3<f32> {
        Point::from(point.coords.map(|x| x.round()))
    }

    #[test]
    fn camera_pos() {
        let camera: Camera = Camera::new(Point3::new(-5.0, 0.0, 0.0),
                                         Vector3::new(1.0, 0.0, 0.0),
                                         1.0, 1.0,
                                         2.0, 800);

        assert_eq!(camera.pos(), Point3::new(-5.0, 0.0, 0.0));
    }

    #[test]
    fn camera_matrix1() {
        let camera: Camera = Camera::new(Point3::new(-5.0, 0.0, 0.0),
                                         Vector3::new(1.0, 0.0, 0.0),
                                         1.0, 1.0,
                                         2.0, 800);

        let point = Point3::new(0.0, 0.0, 4.0);
        let point = camera.matrix * point;
        let point = round(point); // round to avoid errors
        assert_eq!(point, Point3::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn camera_matrix2() {
        let camera: Camera = Camera::new(Point3::new(-5.0, 0.0, 0.0),
                                         Vector3::new(1.0, 0.0, 0.0),
                                         1.0, 1.0,
                                         2.0, 800);

        let point = Point3::new(4.0, 0.0, 0.0);
        let point = camera.matrix * point;
        let point = round(point); // round to avoid errors
        assert_eq!(point, Point3::new(-5.0, 0.0, -4.0));
    }

    #[test]
    fn camera_project1() {
        let camera: Camera = Camera::new(Point3::new(-5.0, 0.0, 0.0),
                                         Vector3::new(1.0, 0.0, 0.0),
                                         1.0, 1.0,
                                         2.0, 800);

        let point = camera.project(400, 400);
        let point = round(point); // round to avoid errors
        assert_eq!(point, Point3::new(-4.0, 0.0, 0.0));
    }

    #[test]
    fn camera_project2() {
        let camera: Camera = Camera::new(Point3::new(-5.0, 0.0, 0.0),
                                         Vector3::new(1.0, 0.0, 0.0),
                                         1.0, 1.0,
                                         2.0, 800);

        let point = camera.project(0, 0);
        let point = round(point); // round to avoid errors
        assert_eq!(point, Point3::new(-4.0, 1.0, 1.0));
    }
}
