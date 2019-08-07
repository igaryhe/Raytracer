use std::ops::{Add, Sub, Mul, Div, Index, Neg};
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub e: [f32; 3]
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {Vec3 {e: [e0, e1, e2]}}
    pub fn x(&self) -> f32 {self.e[0]}
    pub fn y(&self) -> f32 {self.e[1]}
    pub fn z(&self) -> f32 {self.e[2]}
    pub fn r(&self) -> f32 {self.e[0]}
    pub fn g(&self) -> f32 {self.e[1]}
    pub fn b(&self) -> f32 {self.e[2]}
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]]}
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]]}
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]]}
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {e: [self.e[0] * other, self.e[1] * other, self.e[2] * other]}
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {e: [self * other.e[0], self * other.e[1], self * other.e[2]]}
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {e: [self.e[0] / other.e[0], self.e[1] / other.e[1], self.e[2] / other.e[2]]}
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        Self {e: [self.e[0] / other, self.e[1] / other, self.e[2] / other]}
    }
}

impl Index<u8> for Vec3 {
    type Output = f32;
    fn index(&self, i: u8) -> &f32{
        match i {
            0 => &self.e[0],
            1 => &self.e[1],
            2 => &self.e[2],
            _ => &0.0
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {e: [-self.e[0], -self.e[1], -self.e[2]]}
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.e[0] == other.e[0] && self.e[1] == other.e[2] && self.e[2] == other.e[2]
    }
}

impl Vec3 {
    pub fn squared_length(&self) -> f32 {self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)}
    pub fn length(&self) -> f32 {self.squared_length().sqrt()}
    pub fn unit_vector(&self) -> Vec3 {
        Vec3 {e: [self.e[0] / self.length(), self.e[1] / self.length(), self.e[2] / self.length()]}
    }
    pub fn dot(&self, other: &Vec3) -> f32 {
        (self.e[0] * other.e[0] + self.e[1] * other.e[1] +self.e[2] * other.e[2])
    }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {e: [self.e[1] * other.e[2] - self.e[2] * other.e[1],
                  self.e[2] * other.e[0] - self.e[0] * other.e[2],
                  self.e[0] * other.e[1] - self.e[1] * other.e[0]]}
    }
    pub fn sqrt(&self) -> Vec3 {
        Vec3 {e: [self.e[0].sqrt(), self.e[1].sqrt(), self.e[2].sqrt()]}
    }
    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(n) * (*n)
    }
    pub fn refract(&self, n: &Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
        let uv = self.unit_vector();
        let dt = uv.dot(n);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discriminant > 0.0 {
            *refracted = ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt();
            true
        } else {
            false
        }
    }
    pub fn colorize(&mut self) {
        self.e[0] = self.e[0].floor();
        self.e[1] = self.e[1].floor();
    }
}
