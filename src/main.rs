pub mod vec3;
pub mod ray;
pub mod object;
pub mod io;
pub mod camera;
pub mod material;


use std::time::Instant;
use rayon::prelude::*;
use rand::random;
use crate::io::gen_png;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::camera::Camera;
use crate::material::{Scatterable, Material};
use crate::object::{Hitable, sphere::Sphere, list::HitableList};

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p = 2.0*Vec3::new(random::<f64>(), random::<f64>(), random::<f64>())-Vec3::new(1.0, 1.0, 1.0);
    while p.squared_length() >= 1.0 {
        p = 2.0*Vec3::new(random::<f64>(), random::<f64>(), random::<f64>())-Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

fn color(r: &Ray, world: &impl Hitable, depth: u16) -> Vec3 {
    if let Some(rec) = world.hit(&r, 0.0001, core::f64::MAX) {
        let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation = Vec3::new(0.0, 0.0, 0.0);
        if depth < 50 && rec.mat.scatter(r,&rec, &mut attenuation, &mut scattered) {
            attenuation*color(&scattered, world, depth+1)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = Vec3::make_unit_vec(r.direction());
        let t = 0.5*(unit_direction.y()+1.0);
        (1.0-t)*Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0)
    }
}

fn gen_scene() -> HitableList {
    let mut world = HitableList::new();
    world.add(Sphere::new(Vec3::new(0.0, -1000.0, -1.0), 1000.0, Material::Lambertian{albedo: Vec3::new(0.5, 0.5, 0.5)}));
    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat = random::<f64>();
            let center = Vec3::new(a as f64 + random::<f64>()*0.9, 0.2, b as f64 + random::<f64>()*0.9);
            if (center-Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    world.add(Sphere::new(center, 0.2, Material::Lambertian{albedo: Vec3::new(random::<f64>()*random::<f64>(), random::<f64>()*random::<f64>(), random::<f64>()*random::<f64>())}));
                } else if choose_mat < 0.95 {
                    world.add(Sphere::new(center, 0.2, Material::Metal{albedo: Vec3::new(0.5*(1.0+random::<f64>()), 0.5*(1.0+random::<f64>()), 0.5*(1.0+random::<f64>())), fuzz: 0.5*random::<f64>()}));
                } else {
                    world.add(Sphere::new(center, 0.2, Material::Dielectri{ri: 1.5}));
                }
            }
        }
    }
    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Material::Dielectri{ri: 1.5}));
    world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Material::Lambertian{albedo: Vec3::new(0.4, 0.2, 0.1)}));
    world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Material::Metal{albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0}));
    world
}

fn main() {
    let start_time = Instant::now();
    println!("ray tracer starts working");

    let nx = 2000;
    let ny = 1000;
    let ns = 200;
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus= 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
       lookfrom,
       lookat,
       Vec3::new(0.0,1.0, 0.0),
       20.0, nx as f64 / ny as f64,
       aperture, dist_to_focus);
    let world = gen_scene();
    let scene: Vec<Vec<Vec3>> = (0..ny).into_par_iter().map(|y_rev| {
        let y = ny as f64 - y_rev as f64 - 1.0;
        let row: Vec<Vec3> = (0..nx).into_par_iter().map(|x| {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (x as f64 + random::<f64>())/ nx as f64;
                let v = (y as f64 + random::<f64>())/ ny as f64;
                let r = camera.get_ray(u, v);
                col += color(&r, &world, 0);
            }
            col/= ns as f64;
            col.e[0] = col.e[0].sqrt();
            col.e[1] = col.e[1].sqrt();
            col.e[2] = col.e[2].sqrt();
            col*255.99
        }).collect();
        row
    }).collect();

    gen_png(scene, "img.png").expect("Fail to generate image");
    
    println!("cost {:?} seconds", start_time.elapsed().as_secs());
}
