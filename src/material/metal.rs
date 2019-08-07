use crate::structure::*;
use crate::hitable::HitRecord;

use super::Scatterable;
use super::random_in_unit_sphere;

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f32
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal {albedo, fuzz}
    }
}

impl Scatterable for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray)
               -> bool {
        let reflected = r_in.direction().unit_vector().reflect(&rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
        *attenuation = self.albedo;
        scattered.direction().dot(&rec.normal) > 0.0
    }
}
