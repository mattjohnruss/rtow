mod vec3;
mod colour;
mod ray;

use std::io::prelude::*;
use crate::vec3::*;
use crate::ray::*;

fn render() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - Vec3::new(0.0, 0.0, focal_length) - horizontal / 2.0 - vertical / 2.0;

    fn ray_colour(ray: Ray) -> Colour {
        let unit_direction = ray.direction().unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
    }

    // Header
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    // Render
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        std::io::stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_colour = ray_colour(ray);

            colour::write(&mut std::io::stdout(), pixel_colour)
                .expect("failed to write pixel colour to stdout");
        }
    }

    eprintln!("Done");
}

fn main() {
    render();
}
