extern crate nalgebra;

use nalgebra::Vector3;
use std::io::{self, Write};
use std::fs::File;


/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn write_color(img: &mut File, pixel_color: &Vector3<f64>) -> io::Result<()>{
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    let rbyte  = (255.999 * r) as i32;
    let gbyte  = (255.999 * g) as i32;
    let bbyte  = (255.999 * b) as i32;

    writeln!(img, "{} {} {}", rbyte, gbyte, bbyte)
}
