mod colour;
mod hit;
mod ray;
mod sphere;
mod vec3;

use crate::ray::Ray;
use crate::vec3::{Vec3, Point3, Colour};
use crate::hit::{Hittable, HittableList};
use crate::sphere::Sphere;
use std::io::prelude::*;
use std::ops::{Add, Mul};

fn lerp<T>(s: f64, t_1: T, t_2: T) -> T
where T: Add<Output = T>,
      f64: Mul<T, Output = T>
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
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    let mut world = HittableList::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - Vec3::new(0.0, 0.0, focal_length) - horizontal / 2.0 - vertical / 2.0;

    // Header
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    // Render
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        std::io::stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_colour = ray_colour(ray, &world);

            colour::write(&mut std::io::stdout(), pixel_colour)
                .expect("failed to write pixel colour to stdout");
        }
    }

    eprintln!("Done");
}

fn main() {
    render();
}
