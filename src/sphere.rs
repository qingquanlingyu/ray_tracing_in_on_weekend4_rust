use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::rtweekend::{Vec3,Point3};
use crate::material::Material;
use crate::aabb::AABB;
pub struct Sphere<T:Material> {
    center: Point3,
    radius: f64,
    material: T,
    bbox: AABB
}

impl<T:Material> Sphere<T> {
    pub fn new(center: Point3, radius: f64, material: T) -> Self {
        let rvec = Vec3::new(radius,radius,radius);
        let bbox = AABB::new_with_point(center-rvec, center+rvec);
        Sphere { center, radius, material, bbox }
    }
}

impl<T:Material> Hitable for Sphere<T> {
    fn hit(&self, r: &Ray, ray_t:&mut Interval) -> Option<HitRecord> {
        //println!("尝试击中球{},AABB为{:?},光线为{:?}",self.radius, self.bounding_box(),r);
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
    fn bounding_box(&self)->AABB{
        self.bbox
    }
}
