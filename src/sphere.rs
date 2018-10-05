use vector::Vector3;
use ray::Ray;
use hitable::{HitRecord, Hittable};
use material::Material;

#[derive(Debug)]
pub struct Sphere {
    pub center: Vector3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(cen: Vector3, r: f64, material: Material) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            material: material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vector3 = ray.origin() - self.center;
        let a: f64 = ray.direction().dot(&ray.direction());
        let b: f64 = 2.0 * oc.dot(&ray.direction());
        let c: f64 = oc.dot(&oc) - self.radius * self.radius;
        let discriminant: f64 = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let t: f64 = (0.0 - b - discriminant.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                rec.t = t;
                rec.p = ray.point_at_parameter(t);
                // The length p - c would be the radius
                rec.normal = (rec.p - self.center)/self.radius;
                rec.material = self.material;
                return true;
            }
            let t: f64 = (0.0 - b + discriminant.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                rec.t = t;
                rec.p = ray.point_at_parameter(t);
                // The length p - c would be the radius
                rec.normal = (rec.p - self.center)/self.radius;
                rec.material = self.material;
                return true;
            }
        }
        false
    }
}