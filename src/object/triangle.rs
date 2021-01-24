extern crate nalgebra as na;

use std::cmp::Ordering;

use na::*;
use na::geometry::Point3;

use crate::types::*;
use super::{Surface, bound::*};

pub struct Triangle {
    pub v1: usize, // Handles to 3 vertices.
    pub v2: usize,
    pub v3: usize,

    normal: Unit3f, // Precalculated normal vector.
    area: f32, // Precalculated area for barycentric calculations.

    texture: Box<dyn Fn(f32, f32, f32) -> Texture> // Texture map.
                                                   // Uses barycentric coordinates as input.
}

pub struct TriangleMesh {
    pub vertices: Vec<Point3f>,
    pub tris: Vec<Triangle>
}

fn tri_area(a: &Point3f, b: &Point3f, c: &Point3f) -> f32 {
    let prlg_area: f32 = (b - a).cross(&(c - a)).norm();
    prlg_area / 2.0
}

impl Triangle {
    fn vertex1<'a>(&self, vertices: &'a Vec<Point3f>) -> &'a Point3f { &vertices[self.v1] }
    fn vertex2<'a>(&self, vertices: &'a Vec<Point3f>) -> &'a Point3f { &vertices[self.v2] }
    fn vertex3<'a>(&self, vertices: &'a Vec<Point3f>) -> &'a Point3f { &vertices[self.v3] }

    // Conversion of barycentric coordinates to
    // a point on the triangle.
    fn from_bary(&self, vertices: &Vec<Point3f>, t: f32, u: f32, v: f32) -> Point3f {
        Point::from(t * self.vertex1(vertices).coords + u * self.vertex2(vertices).coords + v * self.vertex3(vertices).coords)
    }

    // Conversion of a point to barycentric coordinates.
    fn to_bary(&self, vertices: &Vec<Point3f>, point: Point3f) -> (f32, f32, f32) {
        let t = tri_area(self.vertex2(vertices), self.vertex3(vertices), &point) / self.area;
        let u = tri_area(self.vertex1(vertices), self.vertex3(vertices), &point) / self.area;
        let v = tri_area(self.vertex1(vertices), self.vertex2(vertices), &point) / self.area;

        (t, u, v)
    }

    fn intersect_(&self, vertices: &Vec<Point3f>, ray: Ray) -> Option<(f32, f32, f32)> {
        let vect2_1 = self.vertex2(vertices) - self.vertex1(vertices);
        let vect3_1 = self.vertex3(vertices) - self.vertex1(vertices);

        let p_vect = ray.direction.cross(&vect3_1);
        let det = p_vect.dot(&vect2_1);

        if det.abs() < 1e-3 { return None; }

        let t_vect = ray.origin - self.vertex1(vertices);
        let u = t_vect.dot(&p_vect) / det;

        if u < 0.0 || u > 1.0 { return None; }

        let q_vect = t_vect.cross(&vect2_1);
        let v = ray.direction.dot(&q_vect) / det;

        if v < 0.0 || (u + v) > 1.0 { return None; }

        let t = 1.0 - u - v;

        Some((t, u, v))
    }

    fn intersect(&self, vertices: &Vec<Point3f>, ray: Ray) -> Option<f32> {
        self.intersect_(vertices, ray).map(|(t, u, v)| distance(&ray.origin, &self.from_bary(vertices, t, u, v)))
    }

    fn gettexture(&self, vertices: &Vec<Point3f>, point: Point3f) -> Texture {
        let (t, u, v) = self.to_bary(vertices, point);
        (*self.texture)(t, u, v)
    }
}

#[allow(dead_code)]
impl TriangleMesh {
    pub fn new(vertices: Vec<Point3f>, tris: Vec<(usize, usize, usize, Box<dyn Fn(f32, f32, f32) -> Texture>)>) -> Self {
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

    pub fn new_solid(vertices: Vec<Point3f>, tris: Vec<(usize, usize, usize)>, texture: Texture) -> Self {
        let triangles = tris.into_iter()
                            .map(|(v1, v2, v3)| Triangle {
                                v1: v1,
                                v2: v2,
                                v3: v3,
                                normal: Unit::new_normalize((&vertices[v2] - &vertices[v1]).cross(&(&vertices[v3] - &vertices[v1]))),
                                area: tri_area(&vertices[v1], &vertices[v2], &vertices[v3]),
                                texture: Box::new(move |_, _, _| texture)
                            }).collect();
        TriangleMesh {
            vertices: vertices,
            tris: triangles
        }
    }

    pub fn singleton<F: 'static>(vertex1: Point3f, vertex2: Point3f, vertex3: Point3f, texture: F) -> Self
        where F: Fn(f32, f32, f32) -> Texture
        { TriangleMesh::new(vec![vertex1, vertex2, vertex3], vec![(0, 1, 2, Box::new(texture))]) }

    pub fn singleton_solid(vertex1: Point3f, vertex2: Point3f, vertex3: Point3f, texture: Texture) -> Self
        { TriangleMesh::singleton(vertex1, vertex2, vertex3, move |_, _, _| texture) }


    fn closest_tri(&self, point: Point3f) -> &Triangle {
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

    fn normal(&self, point: Point3f) -> Unit3f {
        self.closest_tri(point).normal
    }

    fn gettexture(&self, point: Point3f) -> Texture {
        self.closest_tri(point).gettexture(&self.vertices, point)
    }

    // Uses Welzl's algorithm to solve the bounding sphere problem
    fn bound(&self) -> Bound {
        fn triangle_sphere(point1: &Point3f, point2: &Point3f, point3: &Point3f) -> (Point3f, f32) {
            let a = point3 - point1;
            let b = point2 - point1;

            let crs = b.cross(&a);

            let to_center = (crs.cross(&b) * a.norm_squared() + a.cross(&crs) * b.norm_squared())
                            / (2.0 * crs.norm_squared());

            let radius = to_center.norm();

            (point1 + to_center, radius)
        }

        fn tetrahedron_sphere(point1: &Point3f, point2: &Point3f, point3: &Point3f, point4: &Point3f) -> (Point3f, f32) {
            let matrix = Matrix4::from_rows(&[point1.to_homogeneous().transpose(),
                                            point2.to_homogeneous().transpose(),
                                            point3.to_homogeneous().transpose(),
                                            point4.to_homogeneous().transpose()]);

            let a = matrix.determinant() * 2.0;
            let mut matrix_mut = matrix.clone();

            let squares = Vector4::new(point1.coords.norm_squared(), point2.coords.norm_squared(), point3.coords.norm_squared(), point4.coords.norm_squared());
            matrix_mut.set_column(0, &squares);
            let center_x = matrix_mut.determinant();

            matrix_mut.set_column(1, &matrix.index((.., 0)));
            let center_y = -matrix_mut.determinant();

            matrix_mut.set_column(2, &matrix.index((.., 1)));
            let center_z = matrix_mut.determinant();

            let center = Point3::new(center_x / a, center_y / a, center_z / a);
            let radius = distance(point1, &center);

            (center, radius)
        }

        fn smallest_sphere(points: Vec<&Point3f>, boundary: Vec<&Point3f>) -> (Point3f, f32) {
            if points.len() == 0 || boundary.len() == 4 {
                match boundary.len() {
                    0 => (Point3::new(0.0, 0.0, 0.0), 0.0),
                    1 => (*boundary[0], 0.0),
                    2 => { let half_span = 0.5 * (boundary[1] - boundary[0]);
                            (*boundary[0] + half_span, half_span.norm()) },
                    3 => triangle_sphere(boundary[0], boundary[1], boundary[2]),
                    4 => tetrahedron_sphere(boundary[0], boundary[1], boundary[2], boundary[3]),
                    _ => unreachable!()
                }
            } else {
                let removed = points[0];
                let points = Vec::from(&points[1..]);

                let bound = smallest_sphere(points.clone(), boundary.clone());
                if distance(&bound.0, removed) < bound.1 { return bound; }

                let mut boundary = boundary.clone();
                boundary.push(removed);

                smallest_sphere(points, boundary)
            }
        }

        extern crate rand;
        use rand::thread_rng;
        use rand::seq::SliceRandom;

        let mut points: Vec<&Point3f> = self.vertices.iter().collect();
        points.shuffle(&mut thread_rng());

        let (center, radius) = smallest_sphere(points, Vec::new());

        Bound { center: center, radius: radius + 1e-3, bypass: false }
    }
}
