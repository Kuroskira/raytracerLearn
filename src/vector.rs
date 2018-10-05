use std::ops::{Add, Div, Mul, Neg, Sub};
use std::f64;

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn r(&self) -> f64 {
        self.x
    }

    pub fn g(&self) -> f64 {
        self.y
    }

    pub fn b(&self) -> f64 {
        self.z
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn length(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Vector3 {
        *self / self.length()
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn print(&self) -> () {
        println!("{:?}", self);
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Vector3) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        other * self
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, other: f64) -> Vector3 {
        if other == 0.0 {
            return Vector3 {
                x: f64::MAX, 
                y: f64::MAX, 
                z: f64::MAX,
            };
        }

        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

#[test]
fn test_gen() {
    let vec: Vector3 = Vector3::new(0.2, 0.4, 0.8);
    vec.print();
}

#[test]
fn test_xyz() {
    let vec: Vector3 = Vector3::new(1.2, 3.4, 6.7);
    assert_eq!(vec.x(), 1.2);
    assert_eq!(vec.y(), 3.4);
    assert_eq!(vec.z(), 6.7);
}

#[test]
fn test_rgb() {
    let vec: Vector3 = Vector3::new(0.2, 0.4, 0.7347845);
    assert_eq!(vec.r(), 0.2);
    assert_eq!(vec.g(), 0.4);
    assert_eq!(vec.b(), 0.7347845);
}

#[test]
fn test_add() {
    let vec_1: Vector3 = Vector3::new(0.2, 0.4, 0.7);
    let vec_2: Vector3 = Vector3::new(0.1, 0.3, 0.3);
    let vec_test: Vector3 = Vector3::new(0.30000000000000004, 0.7, 1.0);
    assert_eq!(vec_1 + vec_2, vec_test);
}

#[test]
fn test_sub() {
    let vec_1: Vector3 = Vector3::new(0.2, 0.4, 0.8);
    let vec_2: Vector3 = Vector3::new(0.1, 0.2, 0.4);
    let vec_test: Vector3 = Vector3::new(0.1, 0.2, 0.4);
    assert_eq!(vec_1 - vec_2, vec_test);
}

#[test]
fn test_mul() {
    let vec: Vector3 = Vector3::new(0.2, 0.4, 0.8);
    let vec_mul: Vector3 = Vector3::new(2.0, 1.0, 1.0);
    assert_eq!(vec * 2.0, Vector3::new(0.4, 0.8, 1.6));
    assert_eq!(2.0 * vec, Vector3::new(0.4, 0.8, 1.6));
    assert_eq!(vec * vec_mul, Vector3::new(0.4, 0.4, 0.8));
}

#[test]
fn test_div() {
    let vec: Vector3 = Vector3::new(0.2, 0.4, 0.8);
    let vec_test: Vector3 = Vector3::new(0.1, 0.2, 0.4);
    let vec_inf: Vector3 = Vector3::new(f64::MAX, f64::MAX, f64::MAX);
    assert_eq!(vec / 2.0, vec_test);
    assert_eq!(vec / 0.0, vec_inf);
}

#[test]
fn test_len() {
    let vec: Vector3 = Vector3::new(0.0, 3.0, 4.0);
    assert_eq!(vec.length(), 5.0);
}

#[test]
fn test_unit() {
    let vec: Vector3 = Vector3::new(0.0, 3.0, 4.0);
    let unit_vec: Vector3 = Vector3::new(0.0, 3.0 / 5.0, 4.0 / 5.0);
    assert_eq!(vec.unit_vector(), unit_vec);
}

#[test]
fn test_dot() {
    let vec_1: Vector3 = Vector3::new(0.2, 0.4, 0.7);
    let vec_2: Vector3 = Vector3::new(0.1, 0.3, 0.3);
    assert_eq!(vec_1.dot(&vec_2), 0.35);
}

#[test]
fn test_cross() {
    let vec_1: Vector3 = Vector3::new(3.0, -3.0, 1.0);
    let vec_2: Vector3 = Vector3::new(4.0, 9.0, 2.0);
    let vec_cross: Vector3 = Vector3::new(-15.0, -2.0, 39.0);
    assert_eq!(vec_1.cross(&vec_2), vec_cross);
}
