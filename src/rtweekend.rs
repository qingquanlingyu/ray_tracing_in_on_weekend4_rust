pub const INFINITY:f64 = std::f64::INFINITY;
pub const SPP:i32 = 256;
pub const RR_PROBABILITY:f64 = 0.85;
use rand::Rng;
use crate::hitable::Hitable;
use std::rc::Rc;

use nalgebra::Vector3;
pub type Color = Vector3<f64>;
pub type Point3 = Vector3<f64>;
pub type Vec3 = Vector3<f64>;

pub fn gamma_correction(linear_in:f64)->f64{
    linear_in.powf(1.0/2.2)
}
pub fn length(v: &Vec3) -> f64 {
    (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
}
pub fn unit_vector(v: &Vec3) -> Vec3 {
    return v / length(v);
}

pub fn random_color()->Color{
    let mut rng = rand::thread_rng();
    Color::new(rng.gen::<f64>(),rng.gen::<f64>(),rng.gen::<f64>())
}

pub fn random_color_range(min:f64, max:f64)->Color{
    let mut rng = rand::thread_rng();
    Color::new(rng.gen_range(min..max),rng.gen_range(min..max),rng.gen_range(min..max))
}

pub fn random_on_unit_sphere()->Vec3{
    let mut rng = rand::thread_rng();
    let phi:f64 = (rng.gen_range(0.0..360.0) as f64).to_radians();
    let cos_theta:f64 = rng.gen_range(0.0..=1.0);
    let sin_theta = cos_theta.acos().sin();

    let mut p:Vec3 = Vec3::new(cos_theta*phi.sin(), cos_theta*phi.cos(), sin_theta);
    
    let tmp = rng.gen_range(0..=1);
    //random reverse z
    if tmp == 0{
        return p;
    }
    else{
        p.z = -p.z;
        return p;
    }
}
pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    let theta:f64 = (rng.gen_range(0.0..360.0) as f64).to_radians();
    let r:f64 = rng.gen::<f64>().sqrt();

    Vec3::new(r*theta.cos(), r*theta.sin(), 0.0)
}

pub fn near_zero(v:&Vec3)->bool {
    // Return true if the vector is close to zero in all dimensions.
    let s = 1e-8;
    (f64::abs(v.x) < s) && (f64::abs(v.y) < s) && (f64::abs(v.z) < s)
}

pub fn reflect(v:&Vec3, n:&Vec3)->Vec3{
    v-2.0*v.dot(n)*n
}

pub fn refract(uv:&Vec3, n:&Vec3, etai_over_etat:f64)->Vec3{
    let cos_theta = f64::min((-uv).dot(&n),1.0);
    let r_out_perp: Vec3 = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel: Vec3 = -(1.0- length(&r_out_perp).powi(2)).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

pub fn box_compare(a:&Rc<dyn Hitable>, b:&Rc<dyn Hitable>, axis_index:i8)->bool{
    a.bounding_box().axis(axis_index).min < b.bounding_box().axis(axis_index).min
}
pub fn box_x_compare(a:&Rc<dyn Hitable>, b:&Rc<dyn Hitable>)->bool{
    box_compare(a, b, 0)
}
pub fn box_y_compare(a:&Rc<dyn Hitable>, b:&Rc<dyn Hitable>)->bool{
    box_compare(a, b, 1)
}
pub fn box_z_compare(a:&Rc<dyn Hitable>, b:&Rc<dyn Hitable>)->bool{
    box_compare(a, b, 2)
}