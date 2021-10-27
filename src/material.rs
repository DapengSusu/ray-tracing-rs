use crate::{Color3, hittable::HitRecord, ray::Ray, rtweekend, vec3::Vec3};
// use dyn_clone::DynClone;

// pub trait Material: DynClone {
//     fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color3, Ray)>;
// }

pub trait Material {
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
            // reflected + Vec3::random_in_unit_sphere().multiply_coef(self.fuzz)
            reflected + Vec3::random_in_unit_sphere() * self.fuzz
        );
        let attenuation = self.albedo;

        if Vec3::dot(scattered.direction(), &hit_record.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ir: f64
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self { ir: index_of_refraction }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);

        r0 + (1.0-cosine).powi(5) * (1.0 - r0)
    }
}

impl Clone for Dielectric {
    fn clone(&self) -> Self {
        Self { ir: self.ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color3, Ray)> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = Vec3::unit_vector(*ray_in.direction());
        // let cos_theta = Vec3::dot(&unit_direction.multiply_coef(-1.0), &hit_record.normal).min(1.0);
        let cos_theta = Vec3::dot(&(-unit_direction), &hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract ||
            Self::reflectance(cos_theta, refraction_ratio) > rtweekend::random() {
            Vec3::reflect(&unit_direction, &hit_record.normal)
        } else {
            Vec3::refract(&unit_direction, &hit_record.normal, refraction_ratio)
        };

        Some((Color3::new(1.0, 1.0, 1.0), Ray::new(hit_record.p, direction)))
    }
}
