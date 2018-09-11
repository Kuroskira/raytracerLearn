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
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for i in 0..self.list.len() {
            if self.list[i].hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.p = temp_rec.p;
                rec.t = temp_rec.t;
                rec.normal = temp_rec.normal;
            }
        }
        hit_anything
    }
}
