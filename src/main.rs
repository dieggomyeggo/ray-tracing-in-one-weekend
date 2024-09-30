extern crate nalgebra;

mod color;
mod ray;
mod util;

use nalgebra::Vector3;
use std::fs::File;
use std::io::{self, Write};

fn ray_color(r: &ray::Ray) -> nalgebra::Vector3<f64> {
    let unit_direction = util::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Vector3::new(1.0,1.0,1.0) + a*Vector3::new(0.5,0.7,1.0)
}

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMG_W: i16 = 400;

//calc image height, ensure it's at least 1.0
const IMG_H: i16 = if (IMG_W as f64 / ASPECT_RATIO) < 1.0 {
    1
} else {
    (IMG_W as f64 / ASPECT_RATIO) as i16
};



// Camera
static FOCAL_LENGTH: f64 = 1.0;
static VIEWPORT_H: f64 = 2.0;
static VIEWPORT_W: f64 = VIEWPORT_H * ((IMG_W / IMG_H) as f64);
static CAMERA_CENTER: nalgebra::Vector3<f64> = nalgebra::Vector3::new(0.0,0.0,0.0);

// Calculate the vectors across the horizontal and down the vertical viewport edges.
static VIEWPORT_U: nalgebra::Vector3<f64> = nalgebra::Vector3::new(VIEWPORT_W, 0.0,0.0);
static VIEWPORT_V: nalgebra::Vector3<f64> = nalgebra::Vector3::new(0.0, -VIEWPORT_H,0.0);

// Calculate the horizontal and vertical delta vectors from pixel to pixel.
fn pixel_delta_u() -> Vector3<f64> {
    VIEWPORT_U / (IMG_W as f64)
}
fn pixel_delta_v() -> nalgebra::Vector3<f64> { 
    VIEWPORT_V / (IMG_H as f64)
}

// Calculate the location of the upper left pixel

fn viewport_upper_left() ->Vector3<f64> {
    CAMERA_CENTER 
    - Vector3::new(0.0, 0.0, FOCAL_LENGTH) 
    - (VIEWPORT_U / (2.0)) 
    - (VIEWPORT_V / (2.0))
}
fn pixel00_loc()-> Vector3<f64> { 
    viewport_upper_left() + 
    0.5 * (pixel_delta_u() + pixel_delta_v())
}

fn main() -> io::Result<()> {
    let mut img = File::create("img.ppm")?;

    writeln!(img, "P3")?;
    writeln!(img, "{} {}", IMG_W, IMG_H)?;
    writeln!(img, "255")?;

    for j in 0..IMG_H {
        print!("\rScanlines remaining: {:?}", IMG_H - j);
        io::stdout().flush().unwrap();
        for i in 0..IMG_W {
            let pixel_center = pixel00_loc() + (i as f64 * pixel_delta_u()) + (j as f64 * pixel_delta_v());
            let ray_direction = pixel_center - CAMERA_CENTER;
            let r = ray::Ray::new(CAMERA_CENTER, ray_direction);
            
            let pixel_color = ray_color(&r);
            color::write_color(&mut img, &pixel_color)?;
        }
    }

    println!("\nDone.");

    Ok(())
}
