mod hitable;
mod ray;
mod sphere;
mod rtweekend;
mod interval;
mod camera;
mod material;

use hitable::HittableList;
use std::fs::File;
use sphere::Sphere;
use camera::*;
use std::error::Error;
use rtweekend::*;
use material::*;

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = "1.ppm";

    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8,0.8,0.0));
    let material_center = Lambertian::new(Color::new(0.7,0.3,0.3));
    let material_left = Metal::new(Color::new(0.8,0.8,0.8));
    let material_right = Metal::new(Color::new(0.8,0.6,0.2));

    world.add(Box::new(Sphere::new(Point3::new(0.0,-100.5,-1.0),100.0, material_ground)));
    world.add(Box::new(Sphere::new(Point3::new(0.0,0.0,-1.0),0.5, material_center)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0,0.0,-1.0),0.5, material_left)));
    world.add(Box::new(Sphere::new(Point3::new(1.0,0.0,-1.0),0.5, material_right)));

    let mut file = File::create(file_name)?;
    let mut c:Camera = Camera::new();
    c.render(&mut file, &world)  
}
