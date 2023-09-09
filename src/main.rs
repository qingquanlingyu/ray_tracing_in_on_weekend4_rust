mod hitable;
mod ray;
mod sphere;
mod rtweekend;
mod interval;
mod camera;

use ray::*;
use hitable::HittableList;
use std::fs::File;
use sphere::Sphere;
use std::rc::Rc;
use camera::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = "1.ppm";

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0,0.0,-1.0),0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0,-100.5,-1.0),100.0)));

    let mut file = File::create(file_name)?;
    let mut c:Camera = Camera::new();
    c.render(&mut file, &world)  
}
