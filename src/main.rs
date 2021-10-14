use create_image::camera::Camera;
use create_image::hittable_list::HittableList;
use create_image::rtweekend;
use create_image::vec3::Vec3;
use create_image::{Color3, Point3};
use create_image::{color, ray::Ray};
// use std::f64::consts::FRAC_PI_4;


fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color3 {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_rec) = world.is_hit(ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = hit_rec.material.scatter(ray, &hit_rec) {
            return attenuation * ray_color(&scattered, world, depth-1);
        } else {
            return Color3::new(0.0, 0.0, 0.0);
        }
    }

    let unit_direction = Color3::unit_vector(*ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);

    Color3::new(1.0, 1.0, 1.0).multiply_coef(1.0 - t)
        + Color3::new(0.5, 0.7, 1.0).multiply_coef(t)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u32 = 1200;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 500;
    const MAX_DEPTH: i32 = 50;

    // World
    let world = HittableList::random_scene();

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup, 20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus
    );

    // Render
    let head = format!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    let width_minus_one = IMAGE_WIDTH - 1;
    let height_minus_one = IMAGE_HEIGHT - 1;

    println!("{}", head);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color3::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rtweekend::random()) / width_minus_one as f64;
                let v = (j as f64 + rtweekend::random()) / height_minus_one as f64;

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }

            color::write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone.");
}
