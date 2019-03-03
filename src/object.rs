pub mod sphere;
pub mod list;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Material
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, normal: Vec3, mat: Material) -> Self {
        HitRecord{t, p, normal, mat}
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, min: f64, max: f64) -> Option<HitRecord>;
}