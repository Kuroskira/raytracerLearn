use lambertian::Lambertian;
use material::Material;
use ray::Ray;
use vector::Vector3;

pub struct HitRecord {
    pub t: f64,
    pub p: Vector3,
    pub normal: Vector3,
    pub material: Material,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            material: Material::Lambertian(Lambertian::new(Vector3::new(0.0, 0.0, 0.0))),
        }
    }

    pub fn normal(&self) -> Vector3 {
        self.normal
    }

    pub fn p(&self) -> Vector3 {
        self.p
    }

    pub fn material(&self) -> Material {
        self.material
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

#[test]
fn test_hit_record() {
    let hit_record = HitRecord::new();
    assert_eq!(hit_record.normal(), Vector3::new(0.0, 0.0, 0.0));
}
