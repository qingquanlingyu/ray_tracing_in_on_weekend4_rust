pub const INFINITY:f64 = std::f64::INFINITY;
pub const SPP:i32 = 256;
pub const RR_PROBABILITY:f64 = 0.8;
use rand::Rng;

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

pub fn random_in_unit_sphere()->Vec3{
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

pub fn near_zero(v:&Vec3)->bool {
    // Return true if the vector is close to zero in all dimensions.
    let s = 1e-8;
    (f64::abs(v.x) < s) && (f64::abs(v.y) < s) && (f64::abs(v.z) < s)
}

pub fn reflect(v:&Vec3, n:&Vec3)->Vec3{
    v-2.0*v.dot(n)*n
}