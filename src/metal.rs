use material::Scatterable;
use hitable::HitRecord;
use vector::Vector3;
use ray::Ray;
use lambertian::point_in_unit_sphere;

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    albedo: Vector3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector3, fuzz: f64) -> Metal {
        let f: f64 = if fuzz < 1.0 {fuzz} else {1.0};
        Metal { albedo: albedo, fuzz: f }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        let reflected: Vector3 = reflect(&ray_in.direction().unit_vector(), &rec.normal());
        *scattered = Ray::new(rec.p(), reflected + self.fuzz*point_in_unit_sphere());
        *attenuation = self.albedo;
        scattered.direction().dot(&rec.normal()) > 0.0
    }
}

pub fn reflect(v: &Vector3, n: &Vector3) -> Vector3 {
    *v - 2.0 * v.dot(n) * *n
}

#[test]
fn test_reflect() {
    let v: Vector3 = Vector3::new(1.0, 2.0, 4.0);
    let n: Vector3 = Vector3::new(0.0, 0.0, 0.0);

    assert_eq!(reflect(&v, &n), v);
}