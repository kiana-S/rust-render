extern crate nalgebra as na;

use na::*;
use na::geometry::Point3;

use crate::types::*;
use super::Surface;

pub struct Plane {
    pub center: Point3<f32>,        // Plane origin (used for texture mapping).
    pub normal: Unit<Vector3<f32>>, // Precomputed plane normal.

    x_axis: Vector3<f32>, // Plane x-axis (The 3D direction that corresponds to the x-direction on the plane).
    y_axis: Vector3<f32>, // Plane y-axis (The 3D direction that corresponds to the y-direction on the plane).

    texture: Box<dyn Fn(f32, f32) -> Color> // Texture map.
                                            // Input coordinates are defined in terms of the axes above.
}

#[allow(dead_code)]
impl Plane {
    // Creates a new plane.
    pub fn new<F: 'static>(center: Point3<f32>, x_axis: Vector3<f32>, y_axis: Vector3<f32>, texture: F) -> Self
        where F: Fn(f32, f32) -> Color
    {
        Plane {
            center: center,
            normal: Unit::new_normalize(x_axis.cross(&y_axis)),
            x_axis: x_axis,
            y_axis: y_axis,
            texture: Box::new(texture)
        }
    }

    // Creates a new plane with the normal flipped.
    pub fn new_flip<F: 'static>(center: Point3<f32>, x_axis: Vector3<f32>, y_axis: Vector3<f32>, texture: F) -> Self
        where F: Fn(f32, f32) -> Color
    {
        Plane {
            center: center,
            normal: Unit::new_normalize(y_axis.cross(&x_axis)),
            x_axis: x_axis,
            y_axis: y_axis,
            texture: Box::new(texture)
        }
    }

    // Creates a new plane of a solid color.
    pub fn new_solid(center: Point3<f32>, x_axis: Vector3<f32>, y_axis: Vector3<f32>, color: Color) -> Self
        { Plane::new(center, x_axis, y_axis, move |_, _| color) }

    // Creates a new flipped plane of a solid color.
    pub fn new_solid_flip(center: Point3<f32>, x_axis: Vector3<f32>, y_axis: Vector3<f32>, color: Color) -> Self
        { Plane::new_flip(center, x_axis, y_axis, move |_, _| color) }


    // Creates a new XY-plane with the given texture map.
    pub fn xy<F: 'static + Fn(f32, f32) -> Color>(texture: F) -> Self
        { Plane::new(Point3::origin(), Vector3::x(), Vector3::y(), texture) }

    // Creates a new XZ-plane with the given texture map.
    pub fn xz<F: 'static + Fn(f32, f32) -> Color>(texture: F) -> Self
        { Plane::new(Point3::origin(), Vector3::x(), Vector3::z(), texture) }
}

impl Surface for Plane {
    fn intersect(&self, ray: Ray) -> Option<f32> {

        let d = self.normal.dot(&ray.direction);
        if d < 1e-5 { return None; }

        let t = (self.center - ray.origin).dot(&*self.normal) / d;

        if t >= 0.0 { Some(t) }
        else { None }
    }

    fn normal(&self, _point: Point3<f32>) -> Unit<Vector3<f32>> { self.normal }

    fn getcolor(&self, point: Point3<f32>) -> Color {
        let rel_pos = point - self.center;
        let proj_point3 = rel_pos - (*self.normal * self.normal.dot(&rel_pos));

        let x = proj_point3.dot(&self.x_axis);
        let y = proj_point3.dot(&self.y_axis);

        (*self.texture)(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plane_new() {
        let plane = Plane::xy(|_, _| Color::black());

        assert_eq!(plane.center, Point3::new(0.0, 0.0, 0.0));
        assert_eq!(plane.normal, Unit::new_unchecked(Vector3::z()));
    }

    #[test]
    fn plane_intersect() {
        const N: f32 = 5.0;
        let plane = Plane::xz(|_, _| Color::black());

        let ray = Ray::new(Point3::new(0.0, N, 0.0), Vector3::new(0.0, -1.0, 0.0));

        assert_eq!(plane.intersect(ray), Some(N));
    }

    #[test]
    fn plane_getcolor() {
        const N: f32 = 5.0;
        let plane = Plane::xz(|x, y| Color::new(x, y, 0.0));

        let point = Point3::new(5.0, 7.0, 6.0);

        assert_eq!(plane.getcolor(point), Color::new(5.0, 6.0, 0.0));
    }
}
