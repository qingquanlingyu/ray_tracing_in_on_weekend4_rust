use crate::ray::Ray;
use crate::rtweekend::{Point3,Vec3};
use crate::interval::Interval;
use crate::material::Material;

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
    fn hit(&self, r: &Ray, ray_t:&Interval) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hitable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hitable>) {
        self.objects.push(object);
    }
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
}

impl Hitable for HittableList {
    fn hit(&self, r: &Ray, ray_t:&Interval) -> Option<HitRecord> {
        let mut res: Option<HitRecord> = None;
        let mut closed_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(tmp_rec) = object.hit(r, &&Interval::new_with_val(ray_t.min, closed_so_far)) {
                closed_so_far = tmp_rec.t;
                res = Some(tmp_rec);
            }
        }
        res
    }
}
