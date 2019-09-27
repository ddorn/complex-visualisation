extern crate num;
extern crate image;

use num::complex::Complex;
use image::{GenericImageView, DynamicImage, ImageBuffer, Rgba};


type RPoint = Complex<f64>;
type SPoint = (u32, u32);
/*
type Color = (u8, u8, u8);
type Image = Vec<Color>;


fn print_ppm(image : Vec<Color>, size : (u32, u32)) {
    assert_eq!(image.len(), (size.0 * size.1) as usize);

    // ppm header
    println!("P3");  // specify RGB format
    println!("{} {}", size.0, size.1);  // image dims
    println!("255");  // color range from 0..256

    for y in 0..size.1 {
        for x in 0..size.0 {
            let color = image[(y * size.0 + x) as usize];
            println!("{} {} {}", color.0, color.1, color.2);
        }
    }
}
*/


#[derive(Debug)]
struct Camera {
    center : RPoint,
    height : f64,
    screen_size : SPoint,
}

impl Camera {
    fn width(&self) -> f64 {
        self.height * (self.screen_size.0 as f64) / (self.screen_size.1 as f64)
    }

    fn to_real(&self, sp: SPoint) -> RPoint {
        let x = sp.0;
        let y = self.screen_size.1 - sp.1;

        let mut real = Complex::new(x as f64, y as f64);

        real -= Complex::new(self.screen_size.0 as f64,
                                 self.screen_size.1 as f64) * 0.5;
        real.re *= self.width() / (self.screen_size.0 as f64);
        real.im *= self.height  / (self.screen_size.1 as f64);

        real
    }

    fn to_screen(&self, rp: RPoint) -> Option<SPoint> {
        let centered_x = (rp.re * (self.screen_size.0 as f64) / self.width()).round() as i32;
        let centered_y = (rp.im * (self.screen_size.1 as f64) / self.height).round() as i32;

        let x = centered_x + self.screen_size.0 as i32 / 2;
        let y = centered_y + self.screen_size.1 as i32 / 2;

        if x < 0
            || x >= self.screen_size.0 as i32
            || y <= 0
            || y > self.screen_size.1 as i32 {
            None
        } else {
            Some((x as u32, self.screen_size.1 - y as u32))
        }
    }

    fn number_of_pixels(&self) -> u32 {
        self.screen_size.0 * self.screen_size.1
    }

    /*
    fn real_to_color(&self, z: RPoint) -> (u8, u8, u8) {
        if z.im.abs() > self.height / 2.0
            || z.re.abs() > self.width() / 2.0 {
            return (0,0,0)
        }
        let (r, b) = if z.re < 0.0 {
            ((-z.re * 255.0 / self.width() * 2.0).round() as u8, 0)
        } else {
            (0, (z.re * 255.0 / self.height * 2.0).round() as u8)
        };

        let g = ((z.im + self.height / 2.0) * 255.0 / self.height).round() as u8;

        (r,g,b)
    }
    */
}


fn plot<F>(f: F, camera : &Camera, base_image: &DynamicImage) -> ImageBuffer<image::Rgba<u8>, Vec<u8>>
    where F : Fn(Complex<f64>) -> Complex<f64>
{
    let mut image = image::ImageBuffer::new( camera.screen_size.0, camera.screen_size.1);

    for i in 0..camera.number_of_pixels() {
        let start = (i % camera.screen_size.0, i / camera.screen_size.0);
        let z = camera.to_real(start);

        let fz: RPoint = f(z);

        if let Some((x, y)) = camera.to_screen(fz) {
            let color = base_image.get_pixel(start.0, start.1);
            image[(x, y)] = color;

            for dx in 0..3 {
                if x == 0 || x == image.width()-1 { continue; }
                let x = x + dx - 1;
                for dy in 0..3 {
                    if y == 0 || y == image.height()-1 { continue; }
                    let y = y + dy - 1;

                    let previous = *image.get_pixel(x, y);
                    if previous.0[0..3] == [0u8; 3]{
                        image[(x, y)] = color;
//                    } else {
//                        image[(x, y)] =
                    }
                }
            }
        }
    }
    image
}

fn main() {
    let camera = Camera {
        center: Complex::new(0.0, 0.0),
        height: 4.0,
        screen_size: (780, 780),
    };

    let base_img = image::open("base.png").unwrap();

    let maxi = 20;
    let f = |z| 1.0 / (1.0 - z);
    for i in 0..maxi {
        let prop = i as f64 / (maxi as f64- 1.0);
        let g = |z| z * (1.0 - prop) + f(z) * prop;
        plot(g, &camera, &base_img)
            .save(format!("out{:03}.jpg", i)).ok();

    }
//    print_ppm(image, camera.screen_size);

}
