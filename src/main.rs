use std::f32;

use image;
use rand;
use rayon::prelude::*;

use raytracer::structure::*;
use raytracer::hitable::*;
use raytracer::camera::Camera;
use raytracer::material::*;

fn color(r: &Ray, world: &Hitable, depth: u32) -> Vec3 {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, f32::MAX, &mut rec) {
        let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation = Vec3::new(0.0, 0.0, 0.0);
        if depth < 50 && rec.material().scatter(r, &mut rec,&mut attenuation, &mut scattered) {
            attenuation * color(&scattered, world, depth + 1)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn random_scene() -> HitableList {
    let mut list = HitableList::new();
    list.add(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0,
                         Material::Lambertian(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f32>();
            let center = Vec3::new(a as f32 + 0.9 * rand::random::<f32>(),
                                   0.2, b as f32 + 0.9 * rand::random::<f32>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    list.add(Sphere::new(center, 0.2,
                                         Material::Lambertian(Lambertian::new(
                        Vec3::new(rand::random::<f32>() * rand::random::<f32>(),
                                  rand::random::<f32>() * rand::random::<f32>(),
                                  rand::random::<f32>() * rand::random::<f32>())))));
                } else if choose_mat < 0.95 {
                    list.add(Sphere::new(center, 0.2,
                                         Material::Metal(Metal::new(
                        Vec3::new(0.5 * (1.0 + rand::random::<f32>()),
                                  0.5 * (1.0 + rand::random::<f32>()),
                                  0.5 * (1.0 + rand::random::<f32>())),
                        0.5 * rand::random::<f32>()))));
                } else {
                    list.add(Sphere::new(center, 0.2, Material::Dielectric(Dielectric::new(1.5))));
                }
            }
        }
    }
    list.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0,
                         Material::Dielectric(Dielectric::new(1.5))));
    list.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0,
                         Material::Lambertian(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)))));
    list.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0,
                         Material::Metal(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))));
    list
}

fn write(s: Vec<Vec<Vec3>>, filename: &str) {
    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(s[0].len() as u32, s.len() as u32);
    for (y, row) in s.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            imgbuf.put_pixel(x as u32, y as u32,
                             image::Rgb([pixel.r() as u8, pixel.g() as u8, pixel.b() as u8]));
        }
    }
    imgbuf.save(filename).unwrap();
}

fn main() {
    let nx: u32 = 1200;
    let ny: u32 = 800;
    let ns: u32 = 100;

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_radius = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(lookfrom, lookat, up, 20.0, nx as f32 / ny as f32,
                          aperture, dist_to_radius);
    
    let world = random_scene();
    let scene: Vec<Vec<Vec3>> = (0..ny).into_par_iter().map(|y_rev| {
        let y = ny - y_rev - 1;
        let row = (0..nx).into_par_iter().map(|x| {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (x as f32 + rand::random::<f32>()) / nx as f32;
                let v = (y as f32 + rand::random::<f32>()) / ny as f32;
                let r = cam.get_ray(u, v);
                col = col + color(&r, &world, 0);
            }
            col = col / ns as f32;
            col.sqrt() * 255.99
        }).collect();
        row
    }).collect();
    
    write(scene, "test.png");
}
