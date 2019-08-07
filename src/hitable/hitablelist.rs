use crate::structure::Ray;
use super::*;

#[derive(Debug)]
pub struct HitableList {
    list: Vec<Sphere>
}

impl HitableList {
    pub fn new() -> HitableList {
        let list: Vec<Sphere> = Vec::new();
        HitableList{list}
    }

    pub fn add(&mut self, s: Sphere) {
        self.list.push(s);
    }
    pub fn size(&self) -> usize {
        self.list.len()
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut tmp_rec: HitRecord = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far= t_max;
        for s in self.list.clone() {
            if s.hit(r, t_min, closest_so_far, &mut tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.t();
                rec.t = tmp_rec.t;
                rec.p = tmp_rec.p;
                rec.normal = tmp_rec.normal;
                rec.material = tmp_rec.material;
            }
        }
        hit_anything
    }
}
