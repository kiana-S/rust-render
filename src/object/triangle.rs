extern crate nalgebra as na;

use std::cmp::Ordering;

use na::*;
use na::geometry::Point3;

use crate::types::*;
use super::Surface;

pub struct Triangle {
    pub v1: usize, // Handles to 3 vertices.
    pub v2: usize,
    pub v3: usize,

    normal: Unit<Vector3<f32>>, // Precalculated normal vector.
    area: f32, // Precalculated area for barycentric calculations.

    texture: Box<dyn Fn(f32, f32, f32) -> Color> // Texture map.
                                                 // Uses barycentric coordinates as input.
}

pub struct TriangleMesh {
    pub vertices: Vec<Point3<f32>>,
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

    fn intersect_(&self, vertices: &Vec<Point3<f32>>, ray: Ray) -> Option<(f32, f32, f32)> {
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

        let t = 1.0 - u - v;

        Some((t, u, v))
    }

    fn intersect(&self, vertices: &Vec<Point3<f32>>, ray: Ray) -> Option<f32> {
        self.intersect_(vertices, ray).map(|(t, u, v)| distance(&ray.origin, &self.from_bary(vertices, t, u, v)))
    }

    fn getcolor(&self, vertices: &Vec<Point3<f32>>, point: Point3<f32>) -> Color {
        let (t, u, v) = self.to_bary(vertices, point);
        (*self.texture)(t, u, v)
    }
}

#[allow(dead_code)]
impl TriangleMesh {
    pub fn new(vertices: Vec<Point3<f32>>, tris: Vec<(usize, usize, usize, Box<dyn Fn(f32, f32, f32) -> Color>)>) -> Self {
        let triangles = tris.into_iter()
                            .map(|(v1, v2, v3, f)| Triangle {
                                v1: v1,
                                v2: v2,
                                v3: v3,
                                normal: Unit::new_normalize((&vertices[v2] - &vertices[v1]).cross(&(&vertices[v3] - &vertices[v1]))),
                                area: tri_area(&vertices[v1], &vertices[v2], &vertices[v3]),
                                texture: f
                            }).collect();
        TriangleMesh {
            vertices: vertices,
            tris: triangles
        }
    }

    pub fn new_solid(vertices: Vec<Point3<f32>>, tris: Vec<(usize, usize, usize)>, color: Color) -> Self {
        let triangles = tris.into_iter()
                            .map(|(v1, v2, v3)| Triangle {
                                v1: v1,
                                v2: v2,
                                v3: v3,
                                normal: Unit::new_normalize((&vertices[v2] - &vertices[v1]).cross(&(&vertices[v3] - &vertices[v1]))),
                                area: tri_area(&vertices[v1], &vertices[v2], &vertices[v3]),
                                texture: Box::new(move |_, _, _| color)
                            }).collect();
        TriangleMesh {
            vertices: vertices,
            tris: triangles
        }
    }

    pub fn singleton<F: 'static>(vertex1: Point3<f32>, vertex2: Point3<f32>, vertex3: Point3<f32>, texture: F) -> Self
        where F: Fn(f32, f32, f32) -> Color
        { TriangleMesh::new(vec![vertex1, vertex2, vertex3], vec![(0, 1, 2, Box::new(texture))]) }

    pub fn singleton_solid(vertex1: Point3<f32>, vertex2: Point3<f32>, vertex3: Point3<f32>, color: Color) -> Self
        { TriangleMesh::singleton(vertex1, vertex2, vertex3, move |_, _, _| color) }


    fn closest_tri(&self, point: Point3<f32>) -> &Triangle {
        self.tris.iter()
            .map(move |tri| {

                let rel_pos = point - tri.vertex1(&self.vertices);
                let proj_point3 = rel_pos - (*tri.normal * tri.normal.dot(&rel_pos));

                let (t, u, v) = tri.to_bary(&self.vertices, Point3::from(proj_point3));

                let t = clamp(t, 0.0, 1.0);
                let u = clamp(u, 0.0, 1.0);
                let v = clamp(v, 0.0, 1.0);

                let point_new = tri.from_bary(&self.vertices, t, u, v);

                (tri, distance(&point, &point_new))
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
            .unwrap().0
    }
}

impl Surface for TriangleMesh {
    fn intersect(&self, ray: Ray) -> Option<f32> {
        self.tris.iter()
             .filter_map(|tri| tri.intersect(&self.vertices, ray))
             .min_by(|a, b| a.partial_cmp(&b).unwrap_or(Ordering::Equal))
    }

    fn normal(&self, point: Point3<f32>) -> Unit<Vector3<f32>> {
        self.closest_tri(point).normal
    }

    fn getcolor(&self, point: Point3<f32>) -> Color {
        self.closest_tri(point).getcolor(&self.vertices, point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundcolor(color: Color) -> Color {
        Color::new((color.red * 100.0).round() / 100.0, (color.green * 100.0).round() / 100.0, (color.blue * 100.0).round() / 100.0)
    }

    #[test]
    fn triangle_intersect() {
        let triangle = TriangleMesh::singleton_solid(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0), Point3::new(0.0, 0.0, 1.0), Color::black());

        let ray = Ray::new(Point3::new(0.5, 5.0, 0.3), Vector3::new(0.0, -1.0, 0.0));

        let (t, u, v) = triangle.tris[0].intersect_(&triangle.vertices, ray).unwrap();

        println!("{},{},{}", t, u, v);

        assert!(t >= 0.0 && t <= 1.0);
        assert!(u >= 0.0 && u <= 1.0);
        assert!(v >= 0.0 && v <= 1.0);
    }

    #[test]
    fn triangle_getcolor() {
        let triangle = TriangleMesh::singleton(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0), Point3::new(0.0, 0.0, 1.0), |t, u, v| Color::new(t, u, v));

        let t = 0.4;
        let u = 0.1;
        let v = 1.0 - t - u;

        let point = triangle.tris[0].from_bary(&triangle.vertices, t, u, v);

        assert_eq!(roundcolor(triangle.getcolor(point)), roundcolor(Color::new(t, u, v)));
    }
}
