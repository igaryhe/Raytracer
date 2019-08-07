mod hitrecord;
mod hitablelist;
mod sphere;

pub use hitrecord::HitRecord;
pub use hitablelist::HitableList;
pub use sphere::Sphere;

use crate::structure::Ray;

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
