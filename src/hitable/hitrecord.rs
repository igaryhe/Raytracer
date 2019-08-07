use crate::structure::Vec3;
use crate::material::{Material, Lambertian};

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord{t: 0.0, p: Vec3::new(0.0, 0.0, 0.0), normal: Vec3::new(0.0, 0.0, 0.0),
        material: Material::Lambertian(Lambertian::new(Vec3::new(1.0, 1.0, 1.0)))}
    }
    pub fn normal(&self) -> Vec3 {
        self.normal.clone()
    }
    pub fn p(&self) -> Vec3 {
        self.p.clone()
    }
    pub fn t(&self) -> f32 {
        self.t
    }
    pub fn material(&self) -> Material {
        self.material
    }
}
