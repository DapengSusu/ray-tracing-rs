use std::rc::Rc;

use crate::{
    Color3,
    Point3,
    hittable::{Hit, HitRecord},
    material::{Dielectric, Lambertian, Metal},
    ray::Ray,
    rtweekend,
    sphere::Sphere
};

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

    pub fn random_scene() -> Self {
        let mut world = Self::new();
        let ground_material = Lambertian::new(Color3::new(0.5, 0.5, 0.5));

        world.add(Rc::new(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            Rc::new(ground_material)
        )));

        for a in -11..11 {
            for b in -11..11 {
                let choose_material = rtweekend::random();
                let center = Point3::new(
                    a as f64 + 0.9 * rtweekend::random(),
                    0.2,
                    b as f64 + 0.9 * rtweekend::random()
                );

                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_material < 0.8 {
                        // diffuse
                        let albedo = Color3::random_vec3() * Color3::random_vec3();
                        let sphere_material = Lambertian::new(albedo);

                        world.add(Rc::new(Sphere::new(
                            center,
                            0.2,
                            Rc::new(sphere_material)
                        )));
                    } else if choose_material < 0.95 {
                        // metal
                        let albedo = Color3::random_vec3_in_range(0.5, 1.0);
                        let fuzz = rtweekend::random_in_range(0.0, 0.5);
                        let sphere_material = Metal::new(albedo, fuzz);

                        world.add(Rc::new(Sphere::new(
                            center,
                            0.2,
                            Rc::new(sphere_material)
                        )));
                    } else {
                        // glass
                        let sphere_material = Dielectric::new(1.5);

                        world.add(Rc::new(Sphere::new(
                            center,
                            0.2,
                            Rc::new(sphere_material)
                        )));
                    }
                }
            }
        }

        let material1 = Dielectric::new(1.5);
        let material2 = Lambertian::new(Color3::new(0.4, 0.2, 0.1));
        let material3 = Metal::new(Color3::new(0.7, 0.6, 0.5), 0.0);

        world.add(Rc::new(Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            Rc::new(material1)
        )));

        world.add(Rc::new(Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            Rc::new(material2)
        )));

        world.add(Rc::new(Sphere::new(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            Rc::new(material3)
        )));

        world
    }
}
