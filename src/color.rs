use crate::Color3;

const RATE: f64 = 255.999;

pub fn write_color(pixel_color: Color3) {
    let color = format!("{} {} {}",
        (pixel_color.x() * RATE) as u32,
        (pixel_color.y() * RATE) as u32,
        (pixel_color.z() * RATE) as u32,
    );

    println!("{}", color);
}
