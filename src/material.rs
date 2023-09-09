use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::rtweekend::*;

pub trait Material{
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self { Lambertian { albedo } }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + random_in_unit_sphere();

        if near_zero(&scatter_direction){
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Vec3
}

impl Metal {
    pub fn new(albedo: Color) -> Self { Metal { albedo } }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(&unit_vector(&r_in.dir()), &rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        Some((self.albedo, scattered))
    }
}