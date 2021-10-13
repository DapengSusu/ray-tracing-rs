use crate::{Color3, hittable::HitRecord, ray::Ray, rtweekend, vec3::Vec3};
use dyn_clone::DynClone;

pub trait Material: DynClone {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color3, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color3
}

impl Lambertian {
    pub fn new(albedo: Color3) -> Self {
        Self { albedo }
    }
}

impl Clone for Lambertian {
    fn clone(&self) -> Self {
        Self { albedo: self.albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color3, Ray)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        Some((self.albedo, Ray::new(hit_record.p, scatter_direction)))
    }
}

pub struct Metal {
    pub albedo: Color3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: rtweekend::clamp(fuzz, 0.0, 1.0),
        }
    }
}

impl Clone for Metal {
    fn clone(&self) -> Self {
        Self {
            albedo: self.albedo,
            fuzz: self.fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color3, Ray)> {
        let reflected = Vec3::reflect(&Vec3::unit_vector(*ray_in.direction()), &hit_record.normal);
        let scattered = Ray::new(
            hit_record.p, 
            reflected + Vec3::random_in_unit_sphere().multiply_coef(self.fuzz)
        );
        let attenuation = self.albedo;

        if Vec3::dot(scattered.direction(), &hit_record.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
