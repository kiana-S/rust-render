extern crate nalgebra as na;

use na::*;
use na::geometry::Point3;

use crate::types::*;
use super::Surface;

pub struct Plane {
    pub center: Point3<f32>,
    pub normal: Unit<Vector3<f32>>,

    x_axis: Vector3<f32>,
    y_axis: Vector3<f32>,
    texture: Box<dyn Fn(f32, f32) -> Color>
}

impl Plane {
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

    pub fn new_solid(center: Point3<f32>, x_axis: Vector3<f32>, y_axis: Vector3<f32>, color: Color) -> Self
        { Plane::new(center, x_axis, y_axis, move |_, _| color) }

    pub fn xy<F: 'static + Fn(f32, f32) -> Color>(texture: F) -> Self
        { Plane::new(Point3::origin(), Vector3::x(), Vector3::y(), texture) }

    pub fn xz<F: 'static + Fn(f32, f32) -> Color>(texture: F) -> Self
        { Plane::new(Point3::origin(), Vector3::x(), Vector3::z(), texture) }
}

impl Surface for Plane {
    fn intersect(&self, ray: Ray) -> Option<f32> {

        let d = self.normal.dot(&ray.direction);
        if d < 1e-6 { return None; }

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
