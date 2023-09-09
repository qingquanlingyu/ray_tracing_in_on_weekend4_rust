use crate::hitable::{HitRecord, Hitable};
use crate::ray::{Point3, Ray, Vec3};
use crate::interval::Interval;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
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
                let mut res = HitRecord::new(root, p, normal);
                let outward_normal = (res.p - self.center) / self.radius;
                res.set_face_normal(r, outward_normal);
                return Some(res);
            }
            let root = (-b + sqrtd) / a;
            if ray_t.surrounds(&root) {
                let p = r.at(root);
                let normal = (p - self.center) / self.radius;
                let mut res = HitRecord::new(root, p, normal);
                let outward_normal = (res.p - self.center) / self.radius;
                res.set_face_normal(r, outward_normal);
                return Some(res);
            }
        }
        None
    }
}
