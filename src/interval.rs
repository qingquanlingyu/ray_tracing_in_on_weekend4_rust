use crate::rtweekend::INFINITY;

#[derive(Clone, Copy, Debug)]
pub struct Interval{
    pub min:f64,
    pub max:f64
}

impl Interval{
    pub fn new(min:f64,max:f64)->Self{
        Interval{
            min,
            max
        }
    }
    pub fn new_combine(a:Interval, b:Interval)->Self{
        Interval{min:a.min.min(b.min), max:a.max.max(b.max)}
    }
    pub fn contains(&self, x:&f64)->bool{
        self.min <= *x && *x <= self.max
    }
    pub fn surrounds(&self, x:&f64)->bool{
        self.min < *x && *x < self.max
    }
    pub fn clamp(&self, x:&f64)->f64{
        if *x<self.min{
            self.min
        }
        else if *x>self.max{
            self.max
        }
        else{
            *x
        }
    }
    fn expand(&self, delta:f64)->Self{
        let padding = delta/2.0;
        Interval { min: self.min - padding, max: self.max + padding }
    }
}

pub const EMPTY:Interval = Interval{min:INFINITY, max:-INFINITY};
pub const UNIVERSE:Interval = Interval{min:-INFINITY, max:INFINITY};