use crate::rtweekend::{Point3, Vec3};
#[derive(Debug)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Point3) -> Self {
        Ray { origin, dir }
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.dir
    }
    pub fn origin(&self) -> Point3 {
        self.origin
    }
    pub fn dir(&self) -> Point3 {
        self.dir
    }
}
