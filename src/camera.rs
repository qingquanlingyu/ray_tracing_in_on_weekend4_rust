use rand::{Rng,rngs::ThreadRng};
use crate::ray::*;
use crate::hitable::*;
use crate::rtweekend::SPP;
use std::fs::File;
use std::error::Error;
use std::io::Write;
use nalgebra::Vector3;
use crate::interval::Interval;
use crate::rtweekend::INFINITY;

pub type Color = Vector3<f64>;
#[warn(non_camel_case_types)]
struct iColor(i32,i32,i32);
pub struct Camera{
    image_height:i32,
    camera_center:Point3,
    pixel00_loc:Point3,
    pixel_delta_u:Vec3,
    pixel_delta_v:Vec3,
    image_width:i32,
    spp:i32,
    rng:ThreadRng
}

impl Camera{
    pub fn new()->Self{
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400;
        let image_height = std::cmp::max(((image_width as f64) / aspect_ratio) as i32,1);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let camera_center = Point3::new(0.0, 0.0, 0.0);
    
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
    
        let viewport_upper_left =
            camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let spp = SPP;

        let rng = rand::thread_rng();

        Camera { image_height, camera_center, pixel00_loc, pixel_delta_u, pixel_delta_v, image_width,spp,rng }
        
    }
    fn length(&self, v: &Vec3) -> f64 {
        (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
    }
    fn unit_vector(&self, v: &Vec3) -> Vec3 {
        return v / self.length(v);
    }
    /*
    fn length_squared(&self, v: &Vec3) -> f64 {
        (v.x * v.x + v.y * v.y + v.z * v.z)
    }

    fn random_vec3(&mut self)->Vec3{
        Vec3::new(self.rng.gen::<f64>(),self.rng.gen::<f64>(),self.rng.gen::<f64>())
    }
    fn random_vec3_minmax(&mut self, min:f64, max:f64)->Vec3{
        Vec3::new(self.rng.gen_range(min..max),self.rng.gen_range(min..max),self.rng.gen_range(min..max))
    }
    */
    fn random_in_unit_sphere(&mut self)->Vec3{
        let phi:f64 = (self.rng.gen_range(0.0..360.0) as f64).to_radians();
        let cos_theta:f64 = self.rng.gen_range(0.0..=1.0);
        let sin_theta = cos_theta.acos().sin();

        let mut p:Vec3 = Vec3::new(cos_theta*phi.sin(), cos_theta*phi.cos(), sin_theta);
        
        let tmp = self.rng.gen_range(0..=1);
        //随机取反z
        if tmp == 0{
            return p;
        }
        else{
            p.z = -p.z;
            return p;
        }
    }
    fn random_on_hemisphere(&mut self, normal:&Vec3)->Vec3{
        let on_sphere = self.random_in_unit_sphere();
        if on_sphere.dot(normal)>0.0{
            on_sphere
        }
        else{
            -on_sphere
        }
    }

    fn ray_color<T:Hitable>(&mut self, r: &Ray, world:&T) -> Color {
        if let Some(res) = world.hit(r, &Interval::new_with_val(0.0, INFINITY)) {
            let direction = self.random_on_hemisphere(&res.normal);
            return 0.5*self.ray_color(&Ray::new(res.p,direction), world);
        }
    
        let unit_direction: Vec3 = self.unit_vector(&r.dir());
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn pixel_sample_square(&mut self)->Vec3{
        let px = -0.5 + self.rng.gen::<f64>();
        let py = -0.5 + self.rng.gen::<f64>();
        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }
    fn get_ray(&mut self,i:i32,j:i32)->Ray{
        let pixel_center =
                    self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_direction = pixel_sample - self.camera_center;

        Ray::new(self.camera_center, ray_direction)    
    }
    fn get_color(&self, pixel_color:&Color)->iColor{
        let scale = 1.0/(self.spp as f64);
        let r = pixel_color.x * scale;
        let g = pixel_color.y * scale;
        let b = pixel_color.z *scale;

        let intensity:Interval = Interval::new_with_val(0.000,0.999);
        let r:i32 = (256.0*intensity.clamp(&r)) as i32;
        let g:i32 = (256.0*intensity.clamp(&g)) as i32;
        let b:i32 = (256.0*intensity.clamp(&b)) as i32;
        iColor(r, g, b)
    }
    pub fn render<T:Hitable>(&mut self, file:&mut File, world:&T) -> Result<(), Box<dyn Error>>{
        file.write_all(format!("P3\n{0} {1}\n255\n", self.image_width, self.image_height).as_bytes())?;

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color:Color = Color::new(0.0,0.0,0.0);
                for _ in 0..self.spp{
                    let r: Ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, world);
                }          
                let out = self.get_color(&pixel_color);
                
                file.write_all(format!("{0} {1} {2}\n", out.0, out.1, out.2).as_bytes())?;
            }
        }

        Ok(())
    }
}