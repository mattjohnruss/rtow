#![feature(clamp)]

mod camera;
mod hit;
mod ray;
mod sphere;
mod vec3;
mod material;

use crate::camera::{Camera, ASPECT_RATIO};
use crate::hit::{Hittable, HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Colour, Vec3};
use crate::material::{Lambertian, Metal};
use rand::prelude::*;
use std::io::prelude::*;
use std::ops::{Add, Mul};
use std::rc::Rc;

fn lerp<T>(s: f64, t_1: T, t_2: T) -> T
where
    T: Add<Output = T>,
    f64: Mul<T, Output = T>,
{
    (1.0 - s) * t_1 + s * t_2
}

fn ray_colour(ray: Ray, world: &impl Hittable, recursion_depth: usize) -> Colour {
    if recursion_depth == 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(&ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = hit.material.scatter(&ray, &hit) {
            attenuation * ray_colour(scattered, world, recursion_depth - 1)
        } else {
            Colour::zero()
        }
    } else {
        const BG_COLOUR_1: Colour = Colour::new(1.0, 1.0, 1.0);
        const BG_COLOUR_2: Colour = Colour::new(0.5, 0.7, 1.0);

        let unit_direction = ray.direction().unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        lerp(t, BG_COLOUR_1, BG_COLOUR_2)
    }
}

fn render() {
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_RECURSION_DEPTH: usize = 50;

    let material_ground = Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.0)));
    let material_centre = Rc::new(Lambertian::new(Colour::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Colour::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Colour::new(0.8, 0.6, 0.2), 1.0));

    let mut world = HittableList::new();
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_centre));
    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    let camera = Camera::new();

    // Header
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut rng = rand::thread_rng();
    let uniform_dist = rand::distributions::Uniform::new(0.0, 1.0);

    // Render
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        std::io::stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let mut pixel_colour = Colour::zero();
            for _sample in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.sample(uniform_dist)) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.sample(uniform_dist)) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_colour += ray_colour(ray, &world, MAX_RECURSION_DEPTH);
            }
            pixel_colour.write(&mut std::io::stdout(), SAMPLES_PER_PIXEL)
                .expect("failed to write pixel colour to stdout");
        }
    }

    eprintln!("Done");
}

fn main() {
    render();
}
