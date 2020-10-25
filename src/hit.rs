use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Hit {
    pub(crate) p: Point3,
    pub(crate) normal: Vec3,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

pub fn face_normal(ray: &Ray, outward_normal: &Vec3) -> (bool, Vec3) {
    let front_face = ray.direction().dot(*outward_normal) < 0.0;
    let normal = if front_face {
        *outward_normal
    } else {
        -*outward_normal
    };
    (front_face, normal)
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: vec![],
        }
    }

    pub fn with_object(object: Box<dyn Hittable>) -> Self {
        HittableList {
            objects: vec![object],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut closest_t_max_so_far = t_max;
        let mut closest_hit_so_far = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_min, closest_t_max_so_far) {
                closest_t_max_so_far = hit.t;
                closest_hit_so_far = Some(hit);
            }
        }
        closest_hit_so_far
    }
}
