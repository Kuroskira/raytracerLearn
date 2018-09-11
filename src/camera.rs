extern crate rand;

use ray::Ray;
use vector::Vector3;
use rand::Rng;

pub struct Camera {
    pub origin: Vector3,
    pub lower_left_corner: Vector3,
    pub horizontal: Vector3,
    pub vertical: Vector3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            origin: Vector3::from(0.0, 0.0, 0.0),
            lower_left_corner: Vector3::from(-2.0, -1.0, -1.0),
            horizontal: Vector3::from(4.0, 0.0, 0.0),
            vertical: Vector3::from(0.0, 2.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            a: self.origin,
            b: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin,
        }
    }
}

pub fn random_in_unit_sphere() -> Vector3 {
    let mut rng = rand::thread_rng();
    let mut p: Vector3;
    while {
        p = 2.0 * Vector3::from(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) 
            - Vector3::from(1.0, 1.0, 1.0);
        p.dot(&p) >= 1.0
    } {}
    p
}