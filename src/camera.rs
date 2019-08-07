use std::f32::consts::PI;

use crate::structure::*;

pub fn random_in_unit_disk() -> Vec3 {
    let mut p: Vec3;
    loop {
        p = 2.0 * Vec3::new(rand::random::<f32>(),
                            rand::random::<f32>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if p.dot(&p) < 1.0 { break; }
    }
    p
}

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3,vfov: f32,
               aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        
        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);
        
        let origin = lookfrom;
        let lower_left_corner = lookfrom - focus_dist * (half_width * u + half_height * v + w);
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;
        
        Camera {origin, lower_left_corner, horizontal, vertical,
                u, v, w, lens_radius}
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(self.origin + offset,
                 self.lower_left_corner + s * self.horizontal +
                 t * self.vertical - self.origin - offset)
    }
}
