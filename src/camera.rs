extern crate rand;

use rand::Rng;
use ray::Ray;
use std::f64::consts::PI;
use vector::Vector3;

pub struct Camera {
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    origin: Vector3,
    lens_radius: f64,
    u: Vector3,
    v: Vector3,
    w: Vector3,
}

impl Camera {
    pub fn new(
        lookfrom: Vector3,
        lookat: Vector3,
        vup: Vector3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let lens_radius: f64 = aperture / 2.0;

        let theta: f64 = vfov * PI / 180.0;
        let half_height: f64 = (theta / 2.0).tan();
        let half_width: f64 = aspect * half_height;

        let origin: Vector3 = lookfrom;

        let w: Vector3 = (lookfrom - lookat).unit_vector();
        let u: Vector3 = vup.cross(&w).unit_vector();
        let v: Vector3 = w.cross(&u);

        let lower_left_corner =
            origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal: Vector3 = 2.0 * half_width * focus_dist * u;
        let vertical: Vector3 = 2.0 * half_height * focus_dist * v;

        Camera {
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
            origin: origin,
            lens_radius: lens_radius,
            u: u,
            v: v,
            w: w,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd: Vector3 = self.lens_radius*random_in_unit_disk();
        let offset: Vector3 = self.u * rd.x() + self.v * rd.y();
        Ray::new(self.origin + offset,
                 self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin - offset)
    }
}

fn random_in_unit_disk() -> Vector3 {
    let mut rng = rand::thread_rng();
    let mut p: Vector3;
    while {
        p = 2.0 * Vector3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0)
            - Vector3::new(1.0, 1.0, 0.0);
        p.dot(&p) >= 1.0
    } {}
    p
}
