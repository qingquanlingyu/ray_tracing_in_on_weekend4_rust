use crate::ray::Ray;
use crate::rtweekend::{Point3,Vec3};
use crate::interval::*;
use crate::material::Material;
use crate::aabb::AABB;
use std::rc::Rc;

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: &'a dyn Material
}
impl<'a> HitRecord<'a> {
    pub fn new(t: f64, p: Point3, normal: Vec3, material: &'a dyn Material) -> Self {
        HitRecord {
            t,
            p,
            normal,
            front_face: true,
            material
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.dir().dot(&outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => outward_normal,
            false => -outward_normal,
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, ray_t:&mut Interval) -> Option<HitRecord>;
    fn bounding_box(&self)->AABB;
}

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hitable>>,
    pub bbox: AABB
}

impl HittableList {
    pub fn add(&mut self, object: Rc<dyn Hitable>) {
        self.bbox = AABB::new_combine(self.bbox, object.bounding_box());
        self.objects.push(object);
    }
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
            bbox: AABB::new(EMPTY, EMPTY, EMPTY)
        }
    }
}

impl Hitable for HittableList {
    fn hit(&self, r: &Ray, ray_t:&mut Interval) -> Option<HitRecord> {
        let mut res: Option<HitRecord> = None;
        let mut closed_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(tmp_rec) = object.hit(r, &mut Interval::new(ray_t.min, closed_so_far)) {
                closed_so_far = tmp_rec.t;
                res = Some(tmp_rec);
            }
        }
        res
    }
    fn bounding_box(&self)->AABB{
        self.bbox
    }
}
