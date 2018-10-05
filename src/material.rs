use ray::Ray;
use hitable::HitRecord;
use vector::Vector3;
use lambertian::Lambertian;
use metal::Metal;
use dielectric::Dielectric;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
	Dielectric(Dielectric),
}


impl Scatterable for Material {
	fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
		match *self {
			Material::Lambertian(ref inner) => inner.scatter(ray_in, rec, attenuation, scattered),
			Material::Metal(ref inner) => inner.scatter(ray_in, rec, attenuation, scattered),
			Material::Dielectric(ref inner) => inner.scatter(ray_in, rec, attenuation, scattered),
		}
	}
}

pub trait Scatterable {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool;
}

