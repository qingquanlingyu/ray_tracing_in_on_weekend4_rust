pub const PI:f64 = 3.14159265358979323846264;
pub const INFINITY:f64 = std::f64::INFINITY;
pub const SPP:i32 = 10;

pub fn degrees_to_radians(degrees:&f64)->f64 {
    degrees * PI / 180.0
}