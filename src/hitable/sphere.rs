use crate::structure::*;
use crate::hitable::*;
use crate::material::*;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere {
        Sphere {center, radius, material}
    }

    pub fn center(&self) -> Vec3 { self.center.clone() }
    pub fn radius(&self) -> f32 { self.radius }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center();
        let a = r.direction().dot(&r.direction());
        let b = 2.0 * oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius().powi(2);
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t > t_min && t < t_max {
                rec.t = t;
                rec.p = r.point_at_parameter(rec.t());
                rec.normal = (rec.p() - self.center()) / self.radius;
                rec.material = self.material;
                return true;
            }
            let t = (-b + discriminant.sqrt()) / (2.0 * a);
            if t > t_min && t < t_max {
                rec.t = t;
                rec.p = r.point_at_parameter(rec.t());
                rec.normal = (rec.p() - self.center()) / self.radius;
                rec.material = self.material;
                return true;
            }
        }
        false
    }
}
