use crate::vec3::Vec3;
use crate::ray::Ray;
use super::*;
use crate::material::Material;


pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Material
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Material) -> Self {
        Sphere{center, radius, mat}
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, min: f64, max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = Vec3::dot(r.direction(), r.direction());
        let b = Vec3::dot(oc, r.direction());
        let c = Vec3::dot(oc, oc) - self.radius*self.radius;
        let delta = b*b - a*c;

        if delta > 0.0 {
            let tmp = (-b-delta.sqrt())/a;
            if tmp < max && tmp > min {
                let t = tmp;
                let p = r.point_at_paramter(t);
                let normal = (p-self.center)/self.radius;
                return Some(HitRecord::new(t, p, normal, self.mat));
            }
        }
        None
    }
}