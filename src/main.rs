use std::rc::Rc;

use create_image::{
    Color3,
    Point3,
    vec3::Vec3,
    hittable_list::HittableList,
    ray::Ray,
    camera::Camera,
    rtweekend,
    color,
    material::{Lambertian, Dielectric, Metal},
    sphere::Sphere
};
// use std::f64::consts::FRAC_PI_4;


fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color3 {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_rec) = world.is_hit(ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) =
            hit_rec.material.scatter(ray, &hit_rec)
        {
            return attenuation * ray_color(&scattered, world, depth-1);
        } else {
            return Color3::new(0.0, 0.0, 0.0);
        }
    }

    let unit_direction = Color3::unit_vector(*ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);

    // Color3::new(1.0, 1.0, 1.0).multiply_coef(1.0 - t)
    //     + Color3::new(0.5, 0.7, 1.0).multiply_coef(t)
    Color3::new(1.0, 1.0, 1.0) * (1.0 - t) + Color3::new(0.5, 0.7, 1.0) * t
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
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

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u32 = 1200;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 500;
    const MAX_DEPTH: i32 = 50;

    // World
    let world = random_scene();

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
