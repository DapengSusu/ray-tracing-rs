use std::ops::{Add, AddAssign, Sub, Mul, Div};
use crate::rtweekend;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length_squared(&self) -> f64 {
        // x^2 + y^2 + z^2
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn multiply_coef(&self, coef: f64) -> Self {
        // coef * (x+y+z)
        Self {
            x: self.x * coef,
            y: self.y * coef,
            z: self.z * coef,
        }
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        // x1*x2 + y1*y2 + z1*z2
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: &Vec3, v: Vec3) -> Self {
        // x = y1*z2 - y2*z1
        // y = x2*z1 - x1*z2
        // z = x1*y2 - x2*y1
        Self {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    pub fn unit_vector(v3: Vec3) -> Self {
        v3.multiply_coef(1.0 / v3.length())
    }

    pub fn random_vec3_in_range(min: f64, max: f64) -> Self {
        Vec3::new(
            rtweekend::random_in_range(min, max),
            rtweekend::random_in_range(min, max),
            rtweekend::random_in_range(min, max),
        )
    }

    pub fn random_vec3() -> Self {
        Vec3::new(
            rtweekend::random(),
            rtweekend::random(),
            rtweekend::random(),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_vec3_in_range(-1.0, 1.0);

            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::unit_vector(Self::random_in_unit_sphere())
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}
