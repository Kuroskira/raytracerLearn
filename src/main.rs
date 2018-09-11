extern crate rand;

mod camera;
mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod vector;

use camera::{Camera, random_in_unit_sphere};
use hitable::{HitRecord, Hittable};
use hitable_list::HittableList;
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use vector::Vector3;

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    print!("P3\n{} {}\n255\n", nx, ny);
    let mut world = HittableList::new();
    world.add_sphere(Sphere::new(Vector3::from(0.0, 0.0, -1.0), 0.5));
    world.add_sphere(Sphere::new(Vector3::from(0.0, -100.5, -1.0), 100.0));
    let cam = Camera::new();
    for j in (0..=ny - 1).rev() {
        for i in 0..nx {
            let mut col = Vector3::from(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (i as f64 + rand::thread_rng().gen_range(0.0, 1.0)) / (nx as f64);
                let v = (j as f64 + rand::thread_rng().gen_range(0.0, 1.0)) / (ny as f64);
                let r = cam.get_ray(u, v);
                let _p = r.point_at_parameter(2.0);
                col = col + color(&r, &world);
            }
            col = col / (ns as f64);
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}

fn color(ray: &Ray, world: &Hittable) -> Vector3 {
    let mut rec: HitRecord = HitRecord::new();
    if world.hit(ray, 0.0, std::f64::MAX, &mut rec) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return 0.5 * color(&Ray{a: rec.p, b: target-rec.p}, world);
    } else {
        let unit_direction = ray.direction().unit_vec();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * Vector3::from(1.0, 1.0, 1.0) + t * Vector3::from(0.5, 0.7, 1.0);
    }
}

// fn hit_sphere(center: &Vector3, radius: f64, ray: &Ray) -> f64 {
//     let oc = ray.origin() - *center;
//     let a = ray.direction().dot(&ray.direction());
//     let b = 2.0 * oc.dot(&ray.direction());
//     let c = oc.dot(&oc) - radius * radius;
//     let discriminant = b * b - 4.0 * a * c;
//     if discriminant < 0.0 {
//         -1.0
//     } else {
//         (-b - discriminant.sqrt()) / (2.0 * a)
//     }
// }
