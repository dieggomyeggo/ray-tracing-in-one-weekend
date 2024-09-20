
const IMG_W:i16 = 256;
const IMG_H:i16 = 256;

fn main() {
    println!("P3");
    println!("{} {}", IMG_W, IMG_H);
    println!("255");

    for j in 0..IMG_H {
        for i in 0..IMG_W {
            let r = (i as f64/ (IMG_W - 1) as f64) as f64;
            let g = (j as f64/ (IMG_H - 1)as f64) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{:?} {:?} {:?}", ir, ig, ib);
        }
    }
}
