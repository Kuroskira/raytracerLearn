use vector::Vector3;
use ray::Ray;
use std::ops::Deref;

pub struct HitRecord {
    pub t: f64,
    pub p: Vector3,
    pub normal: Vector3,
}

impl Deref for HitRecord {
    type Target = Self;

    fn deref(&self) -> &Self {
        &self
    }
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
