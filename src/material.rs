use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::object::HitRecord;
use crate::random_in_unit_sphere;


pub trait Scatterable {
    fn scatter(&self, r_in: &Ray, rec: & HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Lambertian{albedo: Vec3},
    Metal{albedo: Vec3, fuzz: f64}, // 0 <= fuzz <= 1
    Dielectri{ri: f64}
}

impl Scatterable for Material {
    fn scatter(&self, r_in: &Ray, rec: & HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian{albedo} => {
                let target = rec.p+rec.normal+random_in_unit_sphere();
                *scattered = Ray::new(rec.p, target-rec.p);
                *attenuation = *albedo;
                true
            }

            Material::Metal{albedo, fuzz} => {
                let reflected = reflect(Vec3::make_unit_vec(r_in.direction()), rec.normal);
                *scattered = Ray::new(rec.p, reflected+ *fuzz*random_in_unit_sphere());
                *attenuation = *albedo;
                Vec3::dot(scattered.direction(), rec.normal) > 0.0
            }

            Material::Dielectri{ri} => {
                let outward_normal: Vec3;
                let reflected = reflect(Vec3::make_unit_vec(r_in.direction()), rec.normal);
                let ni_over_nt: f64;
                *attenuation = Vec3::new(1.0, 1.0, 1.0);
                let cosine: f64;
                let reflect_prob: f64;
                if Vec3::dot(r_in.direction(), rec.normal) > 0.0 {
                    outward_normal = -rec.normal;
                    ni_over_nt = *ri;
                    cosine = ri*Vec3::dot(r_in.direction(), rec.normal)/r_in.direction().length();
                } else {
                    outward_normal = rec.normal;
                    ni_over_nt = 1.0/(*ri);
                    cosine = -1.0 * Vec3::dot(r_in.direction(), rec.normal)/r_in.direction().length();
                }
                let mut tmp = Vec3::new(1.0, 1.0, 1.0);
                if let Some(refracted) = refract(r_in.direction(), outward_normal, ni_over_nt) {
                    reflect_prob = schlick(cosine, *ri);
                    tmp = refracted;
                } else {
                    *scattered = Ray::new(rec.p, reflected);
                    reflect_prob = 1.0;
                }
                if rand::random::<f64>() < reflect_prob {
                    *scattered = Ray::new(rec.p, reflected);
                } else {
                    *scattered = Ray::new(rec.p, tmp);
                }
                true
            }
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0*Vec3::dot(v, n)*n
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = Vec3::make_unit_vec(v);
    let dt = Vec3::dot(uv, n);
    let delta = 1.0 - ni_over_nt*ni_over_nt*(1.0-dt*dt);
    if delta > 0.0 {
        Some(ni_over_nt*(uv-n*dt)-n*delta.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f64, ri: f64) -> f64 {
    let mut r0 = (1.0-ri)/(1.0+ri);
    r0 = r0*r0;
    r0 + (1.0-r0)*(1.0-cosine).powf(5.0)
}