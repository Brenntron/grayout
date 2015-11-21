extern crate image;
extern crate hsl;

use std::path::Path;
use std::f64::consts::PI;
use image::{Rgb, RgbImage, ImageError, DynamicImage};
use hsl::HSL;

fn turn_gray_mostly(buf: &mut RgbImage) {
    for p in buf.pixels_mut() {
        let Rgb { mut data } = *p;
        let hsl = HSL::from_rgb(&data);

        // What if we completely grayified this pixel. What level of gray would
        // we want?  hsl.l is the "lightness", a number from 0.0 to 1.0.
        // Multiply to convert it to the range 0 to 255.
        let gray_value = hsl.l * 255.99;

        // Figure out how much we want to grayify this pixel. It depends solely
        // on the hue; color_factor is the amount of color we want to shine
        // through, from 0 to 1. We allow the greatest amount of color to shine
        // through when the hue is 350°, a bright red.
        const FAVORITE_COLOR: f64 = 350.0;
        const PICKINESS: i32 = 9;
        let angle = (hsl.h - FAVORITE_COLOR) * (PI / 180.0);
        let color_factor = ((angle.cos() + 1.0) / 2.0).powi(PICKINESS);

        // However much color we're including, add the opposite amount of gray.
        let gray_factor = 1.0 - color_factor;

        for i in (0 .. 3) {
            let orig_value = data[i] as f64;
            data[i] = (color_factor * orig_value + gray_factor * gray_value) as u8;
        }
        *p = Rgb { data: data };
    }
}

fn process_image(infile: &Path, outfile: &Path) -> Result<(), ImageError> {
    let img = try!(image::open(infile));
    match img {
        DynamicImage::ImageRgb8(mut buf) => {
            turn_gray_mostly(&mut buf);
            buf.save(outfile).map_err(|err| ImageError::IoError(err))
        },
        _ =>
            Err(ImageError::UnsupportedError(
                "don't know how to grayify this kind of image".to_string()))
    }
}

fn main() {
    process_image(&Path::new("rust.jpg"),
                  &Path::new("rust-out.png")).unwrap();
    
    println!("The picture has been rusted!");
}
