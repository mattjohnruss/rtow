use crate::vec3::*;
use std::io::prelude::*;

pub fn write<W: Write>(out: &mut W, pixel_colour: Colour) -> std::io::Result<()> {
    let r = (255.999 * pixel_colour.x()) as usize;
    let g = (255.999 * pixel_colour.y()) as usize;
    let b = (255.999 * pixel_colour.z()) as usize;

    writeln!(out, "{} {} {}", r, g, b)
}
