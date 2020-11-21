use crate::hit::Hit;
use crate::ray::Ray;
use crate::vec3::{Colour, Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Colour, Ray)>;
}

pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<(Colour, Ray)> {
        let scatter_direction = {
            let s = hit.normal + Vec3::random_on_unit_sphere();
            if s.is_near_zero() {
                hit.normal
            } else {
                s
            }
        };
        Some((self.albedo, Ray::new(hit.p, scatter_direction)))
    }
}

pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: dbg!(fuzz.clamp(0.0, 1.0)),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Colour, Ray)> {
        let reflected = ray.direction().reflect(hit.normal);
        let scattered = Ray::new(hit.p, reflected + self.fuzz * Vec3::random_in_unit_ball());
        if scattered.direction().dot(hit.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
