use create_image::camera::Camera;
use create_image::hittable_list::HittableList;
use create_image::rtweekend;
use create_image::sphere::Sphere;
use create_image::vec3::Vec3;
use create_image::{Color3, Point3};
use create_image::{color, ray::Ray};


fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color3 {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_rec) = world.is_hit(ray, 0.001, f64::INFINITY) {
        let target = hit_rec.p + hit_rec.normal + Vec3::random_unit_vector();

        return ray_color(
            &Ray::new(hit_rec.p, target - hit_rec.p),
            world,
            depth - 1
        ).multiply_coef(0.5);
    }

    let unit_direction = Color3::unit_vector(*ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);

    Color3::new(1.0, 1.0, 1.0).multiply_coef(1.0 - t)
        + Color3::new(0.5, 0.7, 1.0).multiply_coef(t)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

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
