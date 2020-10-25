mod vec3;
mod colour;

use std::io::prelude::*;
use crate::vec3::*;

const IMAGE_WIDTH: usize = 512;
const IMAGE_HEIGHT: usize = 512;

fn render() {
    // Header
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    // Render
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        std::io::stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;
            let pixel_colour = Colour::new(r, g, b);

            colour::write(&mut std::io::stdout(), pixel_colour)
                .expect("failed to write pixel colour to stdout");
        }
    }

    eprintln!("Done");
}

fn main() {
    render();
}
