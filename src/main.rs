#![feature(clamp)]

mod camera;
mod colour;
mod hit;
mod ray;
mod sphere;
mod vec3;

use crate::camera::{Camera, ASPECT_RATIO};
use crate::hit::{Hittable, HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Colour, Vec3};
use rand::prelude::*;
use std::io::prelude::*;
use std::ops::{Add, Mul};

fn lerp<T>(s: f64, t_1: T, t_2: T) -> T
where
    T: Add<Output = T>,
    f64: Mul<T, Output = T>,
{
    (1.0 - s) * t_1 + s * t_2
}

fn ray_colour(ray: Ray, world: &impl Hittable) -> Colour {
    if let Some(hit) = world.hit(&ray, 0.0, f64::INFINITY) {
        0.5 * (hit.normal + Colour::new(1.0, 1.0, 1.0))
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

    let mut world = HittableList::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

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
                pixel_colour += ray_colour(ray, &world);
            }
            colour::write(&mut std::io::stdout(), pixel_colour, SAMPLES_PER_PIXEL)
                .expect("failed to write pixel colour to stdout");
        }
    }

    eprintln!("Done");
}

fn main() {
    render();
}
