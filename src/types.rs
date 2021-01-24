extern crate nalgebra as na;

use std::ops::{Add, Mul};

use na::*;
use na::geometry::Point3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Unit<Vector3<f32>>
}

impl Ray {
    pub fn from_parts(origin: Point3<f32>, direction: Unit<Vector3<f32>>) -> Self {
        Ray {
            origin: origin,
            direction: direction
        }
    }
    pub fn new(origin: Point3<f32>, direction: Vector3<f32>) -> Self { Ray::from_parts(origin, Unit::new_normalize(direction)) }
    pub fn from_points(origin: Point3<f32>, points_to: Point3<f32>) -> Self { Ray::new(origin, points_to - origin) }

    pub fn project(&self, t: f32) -> Point3<f32> { self.origin + t * self.direction.into_inner() }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,

    _private: () // Private field prevents direct construction
}

#[allow(dead_code)]
impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Color {
            red:   if red   < 0.0 { 0.0 } else { red   },
            green: if green < 0.0 { 0.0 } else { green },
            blue:  if blue  < 0.0 { 0.0 } else { blue  },

            _private: ()
        }
    }

    pub fn to_byte_array(&self) -> [u8; 3] {
        let red   = (255.0 * self.red)   as u8;
        let green = (255.0 * self.green) as u8;
        let blue  = (255.0 * self.blue)  as u8;
        [red, green, blue]
    }

    pub fn gray(brightness: f32) -> Self { Color::new(brightness, brightness, brightness) }

    pub fn black() -> Self { Color::gray(0.0) }
    pub fn white() -> Self { Color::gray(1.0) }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Color {
        Color {
            red:   self.red   + rhs.red,
            green: self.green + rhs.green,
            blue:  self.blue  + rhs.blue,
            _private: ()
        }
    }
}

impl Mul for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        Color {
            red:   self.red   * rhs.red,
            green: self.green * rhs.green,
            blue:  self.blue  * rhs.blue,
            _private: ()
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Color {
        Color {
            red:   self.red   * rhs,
            green: self.green * rhs,
            blue:  self.blue  * rhs,
            _private: ()
        }
    }
}
