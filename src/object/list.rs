use super::*;
use super::sphere::Sphere;
use crate::ray::Ray;

pub struct HitableList {
    list: Vec<Sphere>
}

impl HitableList {
    pub fn new() -> Self {
        HitableList{list: vec![]}
    }

    pub fn add(&mut self, s: Sphere) {
        self.list.push(s);
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, min: f64, max: f64) -> Option<HitRecord> {
        let mut closest_so_far = max;
        let mut result: Option<HitRecord> = None;
        for s in &self.list {
            let tmp_rec = s.hit(r, min, closest_so_far);
            if let Some(rec) = tmp_rec {
                closest_so_far = rec.t;
                result = Some(rec);
            }
        }
        result
    }
}