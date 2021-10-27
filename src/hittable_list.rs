use std::rc::Rc;

use crate::{hittable::{Hit, HitRecord}, ray::Ray};

pub struct HittableList {
    pub hittables_vec: Vec<Rc<dyn Hit>>
}

impl HittableList {
    pub fn new() -> Self {
        Self { hittables_vec: Vec::new() }
    }

    pub fn add(&mut self, item: Rc<dyn Hit>) {
        self.hittables_vec.push(item);
    }

    pub fn del(&mut self) -> Option<Rc<dyn Hit>> {
        if let Some(item) = self.hittables_vec.pop() {
            Some(item)
        } else {
            None
        }
    }

    pub fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = HitRecord::new();
        let mut closest_so_far = t_max;
        let mut hit_anything = false;

        for hittable_obj in &self.hittables_vec {
            if let Some(hit_rec) = hittable_obj.hit(ray, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = hit_rec.t;
                hit_record = hit_rec;
            }
        }

        if hit_anything {
            Some(hit_record)
        } else {
            None
        }
    }
}
