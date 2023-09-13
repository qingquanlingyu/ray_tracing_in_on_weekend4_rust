mod hitable;
mod ray;
mod sphere;
mod rtweekend;
mod interval;
mod camera;
mod material;
mod aabb;
mod bvh;

use hitable::HittableList;
use std::fs::File;
use sphere::Sphere;
use camera::*;
use std::error::Error;
use rtweekend::*;
use material::*;
use rand::Rng;
use std::rc::Rc;
use bvh::BVHNode;

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = "1.ppm";

    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.5,0.5,0.5));
    world.add(Rc::new(Sphere::new(Point3::new(0.0,-1000.0,0.0),1000.0, material_ground)));

    let material1 = Dielectric::new(1.5);
    let material2 = Lambertian::new(Color::new(0.4,0.2,0.1));
    let material3 = Metal::new(Color::new(0.7,0.6,0.5), 0.0);

    world.add(Rc::new(Sphere::new(Point3::new(0.0,1.0,0.0),1.0, material1)));
    world.add(Rc::new(Sphere::new(Point3::new(-4.0,1.0,0.0),1.0, material2)));
    world.add(Rc::new(Sphere::new(Point3::new(4.0,1.0,0.0),1.0, material3)));

    for i in -3..=5{
        for j in -3..=5{
            let mut rng = rand::thread_rng();
            let choose_mat = rng.gen::<f64>();
            let center = Point3::new(i as f64 + 0.9*rng.gen::<f64>(), 0.2, j as f64+ 0.9*rng.gen::<f64>());

            if length(&(center - Point3::new(4.0, 0.2, 0.0))) > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random_color();
                    let sphere_material = Lambertian::new(albedo);
                    world.add(Rc::new(Sphere::new(center,0.2, sphere_material)));
                } 
                else if choose_mat < 0.95{
                    // metal
                    let albedo = random_color_range(0.5,1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Rc::new(Sphere::new(center,0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Rc::new(Sphere::new(center,0.2, sphere_material)));
                }
            }
        }
    }

    let mut file = File::create(file_name)?;
    let mut c:Camera = Camera::new();
    let mut world_bvh = HittableList::new();
    world_bvh.add(Rc::new(BVHNode::new_with_list(world)));
    c.render(&mut file, world_bvh)  
}
