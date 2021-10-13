use crate::Color3;
use crate::rtweekend;

pub fn write_color(pixel_color: Color3, samples_per_pixel: u32) {
    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (pixel_color.x() * scale).sqrt();
    let g = (pixel_color.y() * scale).sqrt();
    let b = (pixel_color.z() * scale).sqrt();

    let color = format!("{} {} {}",
        (256.0 * rtweekend::clamp(r, 0.0, 0.999)) as i32,
        (256.0 * rtweekend::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * rtweekend::clamp(b, 0.0, 0.999)) as i32,
    );

    println!("{}", color);
}
