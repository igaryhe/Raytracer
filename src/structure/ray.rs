use super::Vec3;
#[derive(Debug, Clone)]
pub struct Ray {
    a: Vec3,
    b: Vec3
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray {a, b}
    }
    pub fn origin(&self) -> Vec3 {self.a.clone()}
    pub fn direction(&self) -> Vec3 {self.b.clone()}
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {self.a.clone() + t * self.b.clone()}
}

impl PartialEq for Ray {
    fn eq(&self, other: &Ray) -> bool {
        self.a == other.a && self.b == other.b
    }
}
