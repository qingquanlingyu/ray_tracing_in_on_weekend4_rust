use nalgebra::Vector3;
pub type Point3 = Vector3<f64>;
pub type Vec3 = Vector3<f64>;
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
