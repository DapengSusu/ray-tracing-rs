use std::rc::Rc;

use crate::material::Material;
use crate::{Point3, vec3::Vec3};
use crate::hittable::{Hit, HitRecord};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    // pub material: Box<dyn Material>,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = *ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(&oc, ray.direction());
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b*half_b - a*c;

        if discriminant < 0.0 {
            return None;
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;

            if root < t_min || root > t_max {
                root = (-half_b + sqrtd) / a;
                if root < t_min || root > t_max {
                    return None;
                }
            }

            let mut hit_record = HitRecord::new();

            hit_record.t = root;
            hit_record.p = ray.at(hit_record.t);
            // let outward_normal = (hit_record.p - self.center).multiply_coef(1.0/self.radius);
            let outward_normal = (hit_record.p - self.center) / self.radius;
            hit_record.set_face_normal(ray, &outward_normal);
            // hit_record.material = dyn_clone::clone_box(&*self.material);
            hit_record.material = Rc::clone(&self.material);

            Some(hit_record)
        }
    }
}
