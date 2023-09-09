use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::rtweekend::{Vec3,Point3};
use crate::material::Material;

pub struct Sphere<T:Material> {
    center: Point3,
    radius: f64,
    material: T
}

impl<T:Material> Sphere<T> {
    pub fn new(center: Point3, radius: f64, material: T) -> Self {
        Sphere { center, radius, material }
    }
}

impl<T:Material> Hitable for Sphere<T> {
    fn hit(&self, r: &Ray, ray_t:&Interval) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center;
        let a = r.dir().dot(&r.dir());
        let b = oc.dot(&r.dir());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let sqrtd = discriminant.sqrt();
            let root = (-b - sqrtd) / a;
            if ray_t.surrounds(&root) {
                let p = r.at(root);
                let normal = (p - self.center) / self.radius;
                let mut res = HitRecord::new(root, p, normal, &self.material);
                let outward_normal = (res.p - self.center) / self.radius;
                res.set_face_normal(r, outward_normal);
                return Some(res);
            }
            let root = (-b + sqrtd) / a;
            if ray_t.surrounds(&root) {
                let p = r.at(root);
                let normal = (p - self.center) / self.radius;
                let mut res = HitRecord::new(root, p, normal, &self.material);
                let outward_normal = (res.p - self.center) / self.radius;
                res.set_face_normal(r, outward_normal);
                return Some(res);
            }
        }
        None
    }
}
