use create_image::hittable_list::HittableList;
use create_image::sphere::Sphere;
use create_image::{Color3, Point3, vec3::Vec3};
use create_image::{color, ray::Ray};


fn ray_color(ray: &Ray, world: &HittableList) -> Color3 {
    if let Some(hit_rec) = world.is_hit(ray, 0.0, f64::INFINITY) {
        return (hit_rec.normal + Color3::new(1.0, 1.0, 1.0)).multiply_coef(0.5);
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

    // World
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal.multiply_coef(1.0/2.0)
        - vertical.multiply_coef(1.0/2.0) - Vec3::new(0.0, 0.0, focal_length);

    // Render
    let head = format!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    let width_minus_one = IMAGE_WIDTH - 1;
    let height_minus_one = IMAGE_HEIGHT - 1;

    println!("{}", head);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / width_minus_one as f64;
            let v = j as f64 / height_minus_one as f64;
            let direction = lower_left_corner
                + horizontal.multiply_coef(u) + vertical.multiply_coef(v) - origin;
            let ray = Ray::new(origin, direction);
            let pixel_color = ray_color(&ray, &world);

            color::write_color(pixel_color);
        }
    }

    eprintln!("\nDone.");
}
