use hitable::{HitRecord, Hittable};
use ray::Ray;
use sphere::Sphere;

#[derive(Debug)]
pub struct HittableList {
    pub list: Vec<Sphere>,
}

impl HittableList {
    pub fn new() -> HittableList {
        let spheres_list: Vec<Sphere> = Vec::new();
        HittableList { list: spheres_list }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.list.push(sphere);
    }

    pub fn size(&self) -> usize {
        self.list.len()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;
        for i in 0..self.size() {
            if self.list[i].hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.p = temp_rec.p;
                rec.t = temp_rec.t;
                rec.normal = temp_rec.normal;
                rec.material = temp_rec.material;
            }
        }
        hit_anything
    }
}
