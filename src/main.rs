extern crate nalgebra;

mod color;
mod ray;

use std::fs::File;
use std::io::{self, Write};
use nalgebra::Vector3;

const IMG_W: i16 = 256;
const IMG_H: i16 = 256;

fn main() -> io::Result<()> {
    let mut img = File::create("img.ppm")?;

    writeln!(img, "P3")?;
    writeln!(img, "{} {}", IMG_W, IMG_H)?;
    writeln!(img, "255")?;

    for j in 0..IMG_H {
        print!("\rScanlines remaining: {:?}", IMG_H - j);
        io::stdout().flush().unwrap();
        for i in 0..IMG_W {
            let pixel_color = Vector3::new(
                (i as f64) / (IMG_W as f64 - 1.0),
                (j as f64) / (IMG_H as f64 - 1.0) ,
                0.0);
            color::write_color(&mut img, &pixel_color)?;
        }
    }

    println!("\nDone.");

    Ok(())
}
