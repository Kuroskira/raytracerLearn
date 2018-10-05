extern crate rand;

mod camera;
mod dielectric;
mod hitable;
mod hitable_list;
mod lambertian;
mod material;
mod metal;
mod ray;
mod sphere;
mod vector;

use camera::Camera;
use dielectric::Dielectric;
use hitable::{HitRecord, Hittable};
use hitable_list::HittableList;
use lambertian::Lambertian;
use material::Material;
use material::Scatterable;
use metal::Metal;
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use vector::Vector3;

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    print!("P3\n{} {}\n255\n", nx, ny);
    let mut world = random_scene();
    world.add_sphere(Sphere::new(
        Vector3::new(0.0, 0.0, -1.0),
        0.5,
        Material::Lambertian(Lambertian::new(Vector3::new(0.1, 0.2, 0.5))),
    ));
    world.add_sphere(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        Material::Lambertian(Lambertian::new(Vector3::new(0.8, 0.8, 0.0))),
    ));
    world.add_sphere(Sphere::new(
        Vector3::new(1.0, 0.0, -1.0),
        0.5,
        Material::Metal(Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.3)),
    ));
    world.add_sphere(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        0.5,
        Material::Dielectric(Dielectric::new(1.5)),
    ));
    world.add_sphere(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        -0.45,
        Material::Dielectric(Dielectric::new(1.5)),
    ));

    let cam: Camera = Camera::new(
        Vector3::new(13.0, 2.0, 3.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        (nx as f64) / (ny as f64),
        0.1,
        10.0,
    );
    for j in (0..=ny - 1).rev() {
        for i in 0..nx {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u: f64 = (i as f64 + rand::random::<f64>()) / nx as f64;
                let v: f64 = (j as f64 + rand::random::<f64>()) / ny as f64;
                let r = cam.get_ray(u, v);
                let _p = r.point_at_parameter(2.0);
                col = col + color(&r, &world, 0);
            }
            col = col / ns as f64;
            let ir = (255.99 * col.x.sqrt()) as i32;
            let ig = (255.99 * col.y.sqrt()) as i32;
            let ib = (255.99 * col.z.sqrt()) as i32;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}

fn color(ray: &Ray, world: &Hittable, depth: i32) -> Vector3 {
    let mut rec: HitRecord = HitRecord::new();
    if world.hit(&ray, 0.001, std::f64::MAX, &mut rec) {
        let mut scattered: Ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
        let mut attenuation: Vector3 = Vector3::new(0.0, 0.0, 0.0);
        if depth < 50 && rec
            .material()
            .scatter(ray, &mut rec, &mut attenuation, &mut scattered)
        {
            return attenuation * color(&scattered, world, depth + 1);
        } else {
            return Vector3::new(1.0, 1.0, 1.0);
        }
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
    }
}

fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();

    let mut list: HittableList = HittableList::new();
    list.add_sphere(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian(Lambertian::new(Vector3::new(0.5, 0.5, 0.5))),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen::<f64>();
            let center: Vector3 = Vector3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    list.add_sphere(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian(Lambertian::new(Vector3::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        ))),
                    ));
                } else if choose_mat < 0.95 {
                    //metal
                    list.add_sphere(Sphere::new(
                        center,
                        0.2,
                        Material::Metal(Metal::new(
                            Vector3::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                            ),
                            0.5 * rng.gen::<f64>(),
                        )),
                    ));
                } else {
                    // dielectric
                    list.add_sphere(Sphere::new(
                        center,
                        0.2,
                        Material::Dielectric(Dielectric::new(1.5)),
                    ));
                }
            }

            list.add_sphere(Sphere::new(
                Vector3::new(0.0, 1.0, 0.0),
                1.0,
                Material::Dielectric(Dielectric::new(1.5)),
            ));
            list.add_sphere(Sphere::new(
                Vector3::new(-4.0, 1.0, 0.0),
                1.0,
                Material::Lambertian(Lambertian::new(Vector3::new(0.4, 0.4, 0.1))),
            ));
            list.add_sphere(Sphere::new(
                Vector3::new(4.0, 1.0, 0.0),
                1.0,
                Material::Metal(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0)),
            ));
        }
    }

    list
}
