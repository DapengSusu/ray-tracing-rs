use create_image::camera::Camera;
use create_image::hittable_list::HittableList;
use create_image::material::{Dielectric, Lambertian, Metal};
use create_image::rtweekend;
use create_image::sphere::Sphere;
use create_image::{Color3, Point3};
use create_image::{color, ray::Ray};


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
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color3::new(0.8, 0.8, 0.0));
    // let material_center = Dielectric::new(1.5);
    let material_center = Lambertian::new(Color3::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    // let material_left = Metal::new(Color3::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new(Color3::new(0.8, 0.6, 0.2), 0.0);

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(material_ground)
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(material_center)
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(material_left.clone())
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        Box::new(material_left)
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(material_right)
    )));

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
