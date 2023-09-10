use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::rtweekend::*;
use rand::Rng;

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
        let mut scatter_direction = rec.normal + random_on_unit_sphere();

        if near_zero(&scatter_direction){
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz:f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz_: f64) -> Self { 
        Metal { albedo, fuzz:f64::min(fuzz_,1.0) } 
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(&unit_vector(&r_in.dir()), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz*random_on_unit_sphere());
        if scattered.dir().dot(&rec.normal)>0.0{
            Some((self.albedo, scattered))
        }
        else{
            None
        }
    }
}

pub struct Dielectric {
    ir: f64
}

impl Dielectric {
    pub fn new(index_of_refraction:f64) -> Self { 
        Dielectric { ir:index_of_refraction } 
    }
    fn reflectance(&self, cosine:f64, ref_idx:f64)->f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
        r0 = r0*r0;
        return r0 + (1.0-r0)*(1.0 - cosine).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0,1.0,1.0);
        let refraction_ratio = if rec.front_face {1.0/self.ir} else { self.ir };

        let unit_direction:Vec3 = unit_vector(&r_in.dir());
        let cos_theta = f64::min((-unit_direction).dot(&rec.normal),1.0);
        let sin_theta = (1.0-cos_theta*cos_theta).sqrt();

        let mut dir:Vec3 = Vec3::new(0.0,0.0,0.0);
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        if (cannot_refract || self.reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen::<f64>()){
            dir = reflect(&unit_direction, &rec.normal);
        }
        else{
            dir = refract(&unit_direction, &rec.normal, refraction_ratio);
        }
        let scattered = Ray::new(rec.p, dir);
        Some((attenuation, scattered))
    }
}