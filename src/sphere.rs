use crate::hit::{self, Hit, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere {
    centre: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(centre: Point3, radius: f64) -> Self {
        Sphere { centre, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let origin_centre = ray.origin() - self.centre;
        let a = ray.direction().length_squared();
        let half_b = ray.direction().dot(origin_centre);
        let c = origin_centre.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        let hit_in_range = |t| {
            if t > t_min && t < t_max {
                let p = ray.at(t);
                let outward_normal = (p - self.centre) / self.radius;
                let (_front_face, normal) = hit::face_normal(&ray, &outward_normal);
                Some(Hit {
                    p,
                    normal,
                    t,
                    //front_face,
                })
            } else {
                None
            }
        };

        if discriminant >= 0.0 {
            let root = discriminant.sqrt();

            let t_m = (-half_b - root) / a;
            if let Some(record) = hit_in_range(t_m) {
                return Some(record);
            }

            let t_p = (-half_b + root) / a;
            if let Some(record) = hit_in_range(t_p) {
                return Some(record);
            }
        }

        None
    }
}
