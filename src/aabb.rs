use crate::rtweekend::*;
use crate::interval::Interval;
use crate::ray::Ray;
use std::mem::swap;

#[derive(Clone, Copy, Debug)]
pub struct AABB{
    x:Interval,
    y:Interval,
    z:Interval
}

impl AABB{
    pub fn new(x:Interval, y:Interval, z:Interval)->Self{
        AABB{ x, y, z}
    }
    pub fn new_with_point(a:Point3, b:Point3)->Self{
        let x = Interval::new(a.x.min(b.x),a.x.max(b.x));
        let y = Interval::new(a.y.min(b.y),a.y.max(b.y));
        let z = Interval::new(a.z.min(b.z),a.z.max(b.z));
        AABB{ x, y, z}
    }
    pub fn new_combine(a:AABB, b:AABB)->Self{
        AABB{
            x:Interval::new_combine(a.x, b.x), 
            y:Interval::new_combine(a.y, b.y), 
            z:Interval::new_combine(a.z, b.z)
        }
    }
    pub fn axis(&self, n:i8)->&Interval{
        if n == 0{
            &self.x
        }
        else if n == 1{
            &self.y
        }
        else if n == 2{
            &self.z
        }
        else{
            panic!("Unallowed axis!!!")
        }
    }

    pub fn hit(&self, r:&Ray, ray_t:&Interval)->bool{
        let mut rt = ray_t.clone();
        for (i, (inv_d, orig)) in (0..3).zip(vec![1.0/r.dir().x, 1.0/r.dir().y, 1.0/r.dir().z].iter().zip(vec![r.origin().x, r.origin().y, r.origin().z])) {
            let mut t0 = (self.axis(i).min - orig) * inv_d;
            let mut t1 = (self.axis(i).max - orig) * inv_d;

            if *inv_d < 0.0{
                swap(&mut t0, &mut t1)
            }
            rt.min = t0.max(ray_t.min);
            rt.max = t1.min(ray_t.max);

            if ray_t.max <= ray_t.min{
                return false;
            }
        }
        true
    }
}