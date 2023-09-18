use crate::rtweekend::*;

pub trait texture {
    fn value(&self, u:f64, v:f64, p:&Point3)->Color;
}
pub struct SolidColor{
    color:Color
}

impl SolidColor {
    pub fn new(color:Color)->Self{
        SolidColor { color }
    }
    pub fn new_with_rgb(r:f64,g:f64,b:f64)->Self{
        SolidColor { color:Color::new(r,g,b) }
    }
}
impl texture for SolidColor{
    fn value(&self, u:f64, v:f64, p:&Point3)->Color{
        self.color
    }
}