use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::random;
use std::f64::consts::PI;

#[allow(dead_code)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,

    u : Vec3,
    v : Vec3,
    w : Vec3,
    lens_radius : f64,
}

impl Camera {
    pub fn new(lookfrom : Vec3, lookat : Vec3, vup : Vec3, vfov : f64,
               aspect : f64, aperture : f64, focus_dist : f64) -> Camera {

        let lens_radius = aperture / 2.0;
        let theta = vfov * PI / 180.0;
        let half_height = f64::tan(theta/2.0);
        let half_width = aspect * half_height;

        let origin = lookfrom;
        let w = Vec3::make_unit_vec(lookfrom - lookat);
        let u = Vec3::make_unit_vec(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        let lower_left_corner = origin - half_width*focus_dist*u - half_height*focus_dist*v - focus_dist*w;
        let horizontal = 2.0*half_width*focus_dist*u;
        let vertical = 2.0*half_height*focus_dist*v;

        Camera{
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u, v, w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius*random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin - offset
        )
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut p : Vec3;

    loop {
        p = 2.0*Vec3::new(random::<f64>(),random::<f64>(),0.0) -
            Vec3::new(1.0, 1.0,0.0);

        if Vec3::dot(p,p) < 1.0 {
            return p;
        }
    }
}