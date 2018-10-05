extern crate rand;

use material::Scatterable;
use hitable::HitRecord;
use vector::Vector3;
use ray::Ray;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    pub albedo: Vector3,
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Lambertian {
        Lambertian { albedo: albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        let target: Vector3 = rec.p() + rec.normal() + point_in_unit_sphere();
        *scattered = Ray::new(rec.p(), target - rec.p());
        *attenuation = self.albedo;
        true
    }
}

pub fn point_in_unit_sphere() -> Vector3 {
    // Initialising p with a value outside the unit sphere
    let mut p: Vector3 = Vector3::new(2.0, 2.0, 2.0);
    let mut rng = rand::thread_rng();
    while p.dot(&p) >= 1.0 {
        p = 2.0 * Vector3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) - Vector3::new(1.0, 1.0, 1.0);
    }
    p
}

#[test]
fn test_point() {
    let vec: Vector3 = point_in_unit_sphere();
    assert_eq!(vec.dot(&Vector3::new(0.0, 0.0, 0.0)), 0.0);
}