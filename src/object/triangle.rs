extern crate nalgebra as na;

use na::*;
use na::geometry::Point3;

use crate::types::*;
use super::Surface;

pub struct Triangle {
    pub v1: usize, // Handles to 3 vertices.
    pub v2: usize,
    pub v3: usize,

    area: f32, // Precalculated area for barycentric calculations.

    texture: Box<dyn Fn(f32, f32, f32) -> Color> // Texture map.
                                                 // Uses barycentric coordinates as input.
}

pub struct TriangleMesh {
    pub points: Vec<Point3<f32>>,
    pub tris: Vec<Triangle>
}

fn tri_area(a: &Point3<f32>, b: &Point3<f32>, c: &Point3<f32>) -> f32 {
    let prlg_area: f32 = (b - a).cross(&(c - a)).norm();
    prlg_area / 2.0
}

impl Triangle {
    fn vertex1<'a>(&self, vertices: &'a Vec<Point3<f32>>) -> &'a Point3<f32> { &vertices[self.v1] }
    fn vertex2<'a>(&self, vertices: &'a Vec<Point3<f32>>) -> &'a Point3<f32> { &vertices[self.v2] }
    fn vertex3<'a>(&self, vertices: &'a Vec<Point3<f32>>) -> &'a Point3<f32> { &vertices[self.v3] }

    // Conversion of barycentric coordinates to
    // a point on the triangle.
    fn from_bary(&self, vertices: &Vec<Point3<f32>>, t: f32, u: f32, v: f32) -> Point3<f32> {
        Point::from(t * self.vertex1(vertices).coords + u * self.vertex2(vertices).coords + v * self.vertex3(vertices).coords)
    }

    // Conversion of a point to barycentric coordinates.
    fn to_bary(&self, vertices: &Vec<Point3<f32>>, point: Point3<f32>) -> (f32, f32, f32) {
        let t = tri_area(self.vertex2(vertices), self.vertex3(vertices), &point) / self.area;
        let u = tri_area(self.vertex1(vertices), self.vertex3(vertices), &point) / self.area;
        let v = tri_area(self.vertex1(vertices), self.vertex2(vertices), &point) / self.area;

        (t, u, v)
    }

    fn intersect(&self, vertices: &Vec<Point3<f32>>, ray: Ray) -> Option<f32> {
        let vect2_1 = self.vertex2(vertices) - self.vertex1(vertices);
        let vect3_1 = self.vertex3(vertices) - self.vertex1(vertices);

        let p_vect = ray.direction.cross(&vect3_1);
        let det = p_vect.dot(&vect2_1);

        if det.abs() < 1e-5 { return None; }

        let t_vect = ray.origin - self.vertex1(vertices);
        let u = t_vect.dot(&p_vect) / det;

        if u < 0.0 || u > 1.0 { return None; }

        let q_vect = t_vect.cross(&vect2_1);
        let v = ray.direction.dot(&q_vect) / det;

        if v < 0.0 || (u + v) > 1.0 { return None; }

        let t = vect3_1.dot(&q_vect) / det;

        // Convert from barycentric coordinates
        Some(distance(&ray.origin, &self.from_bary(vertices, t, u, v)))
    }

    fn normal(&self, vertices: &Vec<Point3<f32>>, _point: Point3<f32>) -> Unit<Vector3<f32>> {
        Unit::new_normalize((self.vertex2(vertices) - self.vertex1(vertices)).cross(&(self.vertex3(vertices) - self.vertex1(vertices))))
    }

    fn getcolor(&self, vertices: &Vec<Point3<f32>>, point: Point3<f32>) -> Color {
        // Converting back and forth between barycentric coordinates
        // like this is terrible, but it's necessary for this object to
        // match the interface the other objects use.
        let (t, u, v) = self.to_bary(vertices, point);

        (*self.texture)(t, u, v)
    }
}

impl TriangleMesh {
    pub fn new(points: Vec<Point3<f32>>, tris: Vec<(usize, usize, usize, Box<dyn Fn(f32, f32, f32) -> Color>)>) -> Self {
        let triangles = tris.into_iter()
                            .map(|(v1, v2, v3, f)| Triangle {
                                v1: v1,
                                v2: v2,
                                v3: v3,
                                area: tri_area(&points[v1], &points[v2], &points[v3]),
                                texture: f
                            }).collect();
        TriangleMesh {
            points: points,
            tris: triangles
        }
    }

    pub fn new_solid(points: Vec<Point3<f32>>, tris: Vec<(usize, usize, usize)>, color: Color) -> Self {
        let triangles = tris.into_iter()
                            .map(|(v1, v2, v3)| Triangle {
                                v1: v1,
                                v2: v2,
                                v3: v3,
                                area: tri_area(&points[v1], &points[v2], &points[v3]),
                                texture: Box::new(move |_, _, _| color)
                            }).collect();
        TriangleMesh {
            points: points,
            tris: triangles
        }
    }

    pub fn singleton<F: 'static>(vertex1: Point3<f32>, vertex2: Point3<f32>, vertex3: Point3<f32>, texture: F) -> Self
        where F: Fn(f32, f32, f32) -> Color
        { TriangleMesh::new(vec![vertex1, vertex2, vertex3], vec![(0, 1, 2, Box::new(texture))]) }
