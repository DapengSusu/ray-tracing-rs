use crate::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(
            lookfrom: Point3,
            lookat: Point3,
            vup: Vec3,
            vfov: f64,
            aspect_ratio: f64,
            aperture: f64,
            focus_dist: f64
        ) -> Self {
        let theta = vfov.to_radians(); // vfov: vertical field-of-view in degrees
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);

        let origin = lookfrom;
        let horizontal = u.multiply_coef(viewport_width * focus_dist);
        let vertical = v.multiply_coef(viewport_height * focus_dist);
        let lower_left_corner = origin - horizontal.multiply_coef(1.0/2.0)
            - vertical.multiply_coef(1.0/2.0) - w.multiply_coef(focus_dist);
        let lens_radius = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk().multiply_coef(self.lens_radius);
        let offset = self.u.multiply_coef(rd.x()) + self.v.multiply_coef(rd.y());
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal.multiply_coef(s)
                + self.vertical.multiply_coef(t) - self.origin - offset
        )
    }
}
