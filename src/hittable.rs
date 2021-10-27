use std::rc::Rc;

use crate::Color3;
use crate::material::{Lambertian, Material};
use crate::{Point3, vec3::Vec3};
use crate::ray::Ray;

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3, // 法线
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            // material: Box::new(Lambertian::new(Color3::new(0.0, 0.0, 0.0))),
            material: Rc::new(Lambertian::new(Color3::new(0.0, 0.0, 0.0))),
            t: 0.0,
            front_face: true,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            // (*outward_normal).multiply_coef(-1.0)
            -(*outward_normal)
        }
    }
}
