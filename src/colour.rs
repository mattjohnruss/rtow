use crate::vec3::*;
use std::io::Write;

pub fn write(
    out: &mut impl Write,
    pixel_colour: Colour,
    samples_per_pixel: usize,
) -> std::io::Result<()> {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = scale * pixel_colour.x();
    let g = scale * pixel_colour.y();
    let b = scale * pixel_colour.z();

    writeln!(
        out,
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as usize,
        (256.0 * g.clamp(0.0, 0.999)) as usize,
        (256.0 * b.clamp(0.0, 0.999)) as usize,
    )
}
