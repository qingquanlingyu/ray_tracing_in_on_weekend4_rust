use crate::rtweekend::*;
use crate::hitable::*;
use crate::aabb::AABB;
use crate::ray::Ray;
use crate::interval::Interval;
use rand::Rng;
use std::rc::Rc;

pub struct BVHNode{
    contents: BVHContents,
    bbox:AABB
}

pub enum BVHContents {
    Node { left: Rc<dyn Hitable>, right: Rc<dyn Hitable> },
    Leaf (Rc<dyn Hitable>),
}

impl Hitable for BVHNode{
    fn hit(&self, r: &Ray, ray_t:&mut Interval) -> Option<HitRecord>{
        if !self.bbox.hit(r,ray_t){
            return None;
        }
        match &self.contents{
            BVHContents::Node { left, right } => {
                let hit_left = left.hit(r, ray_t);
                
                let mut hit_right = None;
                if let Some(h) = &hit_left{
                    hit_right = right.hit(r, &mut Interval::new(ray_t.min, h.t));
                }
                else{
                    hit_right = right.hit(r, &mut Interval::new(ray_t.min, ray_t.max));
                }
                
                match (hit_left, hit_right) {
                    (h, None) | (None, h) => h,
                    (Some(hl), Some(hr)) => {
                        if hl.t < hr.t {
                            Some(hl)
                        } else {
                            Some(hr)
                        }
                    }
                }
            },
            BVHContents::Leaf(obj) => {
                obj.hit(r, ray_t)
            }
        }
    }
    fn bounding_box(&self)->AABB{
        return self.bbox;
    }
}

impl BVHNode{
    pub fn new(src_objects: &Vec<Rc<dyn Hitable>>, start:usize, end:usize)->Self{
        let mut objects = src_objects.clone();

        let axis:i32 = rand::thread_rng().gen_range(0..=2);
        let axis = axis as i8;

        let comparator = match axis {
            0=>box_x_compare,
            1=>box_y_compare,
            2=>box_z_compare,
            _other=>box_x_compare
        };

        let object_span = end-start;
        match object_span {
            0=>panic!("Can't create a BVH from zero objects!!!"),
            1=>{
                BVHNode {bbox:objects[start].bounding_box(), contents: BVHContents::Leaf(objects[start].clone())}
            },
            _=>{
                objects[start..end].sort_unstable_by(|a, b| {if comparator(&a, &b) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }});

                let mid = start + object_span/2;
                let left = Rc::new(BVHNode::new(&objects, start, mid));
                let right = Rc::new(BVHNode::new(&objects, mid, end));

                BVHNode {bbox:AABB::new_combine(left.bounding_box(), right.bounding_box()), contents: BVHContents::Node { left, right }}
            }
        }
    }
    pub fn new_with_list(list:HittableList)->Self{
        BVHNode::new(&list.objects, 0, list.objects.len())
    }
}