use rand::Rng;
use crate::ray::*;
use crate::hitable::*;
use crate::rtweekend::SPP;
use std::fs::File;
use std::error::Error;
use std::io::Write;
use crate::interval::Interval;
use crate::rtweekend::*;

#[warn(non_camel_case_types)]
struct IColor(i32,i32,i32);
pub struct Camera{
    image_height:i32,
    camera_center:Point3,
    pixel00_loc:Point3,
    pixel_delta_u:Vec3,
    pixel_delta_v:Vec3,
    image_width:i32,
    spp:i32,
    defocus_angle:f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3
}

impl Camera{
    pub fn new()->Self{
        let aspect_ratio = 16.0/9.0;
        let image_width = 540;
        let image_height = std::cmp::max(((image_width as f64) / aspect_ratio) as i32,1);

        let lookfrom = Point3::new(13.0, 2.0, 3.0);
        let lookat = Point3::new(0.0, 0.0, 0.0);
        let vfov:f64 = 20.0;
        let defocus_angle:f64 = 0.05;
        let focus_dist:f64 = 10.0;

        let camera_center = lookfrom;
        let theta = vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        const VUP:Vec3 = Vec3::new(0.0,1.0,0.0);
        let w = unit_vector(&(lookfrom - lookat));
        let u = unit_vector(&VUP.cross(&w));
        let v = w.cross(&u);
    
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
    
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
    
        let viewport_upper_left =
            camera_center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let spp = SPP;

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;


        Camera { image_height, camera_center, pixel00_loc, pixel_delta_u, pixel_delta_v, image_width, spp, defocus_angle, defocus_disk_u, defocus_disk_v }
        
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
        fn random_on_hemisphere(&mut self, normal:&Vec3)->Vec3{
        let on_sphere = self.random_in_unit_sphere();
        if on_sphere.dot(normal)>0.0{
            on_sphere
        }
        else{
            -on_sphere
        }
    }
    */
    fn ray_color<T:Hitable>(&mut self, r: &Ray, world:&T, depth:i32) -> Color {
        if depth < 3{
            if let Some(rec) = world.hit(r, &mut Interval::new(0.001, INFINITY)) {
                if let Some((attenuation,scattered)) = rec.material.scatter(&r, &rec){
                    let res = self.ray_color(&scattered, world, depth+1);
                    //WHY CANNOT MUL attenuation and res???😡
                    return Color::new(attenuation.x*res.x,attenuation.y*res.y,attenuation.z*res.z);
                }  
            }
        
            let unit_direction: Vec3 = unit_vector(&r.dir());
            let a = 0.5 * (unit_direction.y + 1.0);
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
        }
        else{
            if let Some(rec) = world.hit(r, &mut Interval::new(0.001, INFINITY)) {
                if let Some((attenuation,scattered)) = rec.material.scatter(&r, &rec){
                    let tmp = rand::thread_rng().gen_range(0.0..1.0);
                    if tmp < RR_PROBABILITY
                    {
                        let res = self.ray_color(&scattered, world, depth+1);
                        //WHY CANNOT MUL attenuation and res???😡
                        return Color::new(attenuation.x*res.x,attenuation.y*res.y,attenuation.z*res.z)/RR_PROBABILITY;
                    }
                    else {
                        return Color::new(0.0,0.0,0.0);
                    }
                }  
            }
        
            let unit_direction: Vec3 = unit_vector(&r.dir());
            let a = 0.5 * (unit_direction.y + 1.0);
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)/RR_PROBABILITY
        }
    }

    fn pixel_sample_square(&mut self)->Vec3{
        let mut rng = rand::thread_rng();
        let px = -0.5 + rng.gen::<f64>();
        let py = -0.5 + rng.gen::<f64>();
        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }
    fn defocus_disk_sample(&self) -> Point3{
        let p = random_in_unit_disk();
        self.camera_center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
    fn get_ray(&mut self,i:i32,j:i32)->Ray{
        let pixel_center =
                    self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {self.camera_center} else {self.defocus_disk_sample()};
        let ray_direction = pixel_sample - self.camera_center;

        Ray::new(ray_origin, ray_direction)    
    }
    fn get_color(&self, pixel_color:&Color)->IColor{
        let scale = 1.0/(self.spp as f64);
        let r = gamma_correction(pixel_color.x * scale);
        let g = gamma_correction(pixel_color.y * scale);
        let b = gamma_correction(pixel_color.z *scale);

        let intensity:Interval = Interval::new(0.000,0.999);
        let r:i32 = (256.0*intensity.clamp(&r)) as i32;
        let g:i32 = (256.0*intensity.clamp(&g)) as i32;
        let b:i32 = (256.0*intensity.clamp(&b)) as i32;
        IColor(r, g, b)
    }
    pub fn render<T:Hitable>(&mut self, file:&mut File, world:T) -> Result<(), Box<dyn Error>>{
        file.write_all(format!("P3\n{0} {1}\n255\n", self.image_width, self.image_height).as_bytes())?;

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color:Color = Color::new(0.0,0.0,0.0);
                for _ in 0..self.spp{
                    let r: Ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, &world, 0);
                }          
                let out = self.get_color(&pixel_color);
                
                file.write_all(format!("{0} {1} {2}\n", out.0, out.1, out.2).as_bytes())?;
            }
        }

        Ok(())
    }
}