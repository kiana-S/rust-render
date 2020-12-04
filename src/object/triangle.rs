extern crate nalgebra as na;

use na::*;
use na::geometry::Point3;

use crate::types::*;
use super::Surface;

pub struct Triangle<'a> {
    pub vertex1: &'a Point3<f32>, // References to 3 vertices.
    pub vertex2: &'a Point3<f32>,
    pub vertex3: &'a Point3<f32>,

    area: f32, // Precalculated area for barycentric calculations.

    texture: Box<dyn Fn(f32, f32, f32) -> Color> // Texture map.
                                                 // Uses barycentric coordinates as input.
}

pub struct TriangleMesh<'a> {
    pub points: Vec<Box<Point3<f32>>>,
    pub tris: Vec<Triangle<'a>>
}

fn tri_area(a: &Point3<f32>, b: &Point3<f32>, c: &Point3<f32>) -> f32 {
    let prlg_area: f32 = (b - a).cross(&(c - a)).norm();
    prlg_area / 2.0
}

impl<'a> Triangle<'a> {
    pub fn new<F: 'static>(vertex1: &'a Point3<f32>,
                            vertex2: &'a Point3<f32>,
                            vertex3: &'a Point3<f32>,
                            texture: F) -> Self
        where F: Fn(f32, f32, f32) -> Color
    {
        Triangle {
            vertex1: vertex1,
            vertex2: vertex2,
            vertex3: vertex3,
            area: tri_area(vertex1, vertex2, vertex3),
            texture: Box::new(texture)
        }
    }

    pub fn new_solid(vertex1: &'a Point3<f32>,
                    vertex2: &'a Point3<f32>,
                    vertex3: &'a Point3<f32>, color: Color) -> Self
        { Triangle::new(vertex1, vertex2, vertex3, move |_, _, _| color) }

    // Conversion of barycentric coordinates to
    // a point on the triangle.
    pub fn from_bary(&self, t: f32, u: f32, v: f32) -> Point3<f32> {
        Point::from(t * self.vertex1.coords + u * self.vertex2.coords + v * self.vertex3.coords)
    }

    // Conversion of a point to barycentric coordinates.
    pub fn to_bary(&self, point: Point3<f32>) -> (f32, f32, f32) {
        let t = tri_area(self.vertex2, self.vertex3, &point) / self.area;
        let u = tri_area(self.vertex1, self.vertex3, &point) / self.area;
        let v = tri_area(self.vertex1, self.vertex2, &point) / self.area;

        (t, u, v)
    }
}

impl<'a> TriangleMesh<'a> {
    pub fn new(points: Vec<Box<Point3<f32>>>, tris: Vec<Triangle<'a>>) -> Self {
        TriangleMesh {
            points: points,
            tris: tris
        }
    }
}
