extern crate num_complex;

mod paon;

use paon::Image;
use num_complex::Complex;
use crate::paon::Color;

type RPoint = Complex<f64>;
type SPoint = (u32, u32);


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


fn transform<F>(f: F, camera : &Camera, base_image: &Image) -> Image
    where F : Fn(Complex<f64>) -> Complex<f64>
{
    let black = Color::new(0,0,0);

    let mut image = Image::new( camera.screen_size.0, camera.screen_size.1);

    for i in 0..camera.number_of_pixels() {
        let start = (i % camera.screen_size.0, i / camera.screen_size.0);

        let z = camera.to_real(start);

        let fz: RPoint = f(z);

        if let Some((x, y)) = camera.to_screen(fz) {
            let color = base_image.at(start);
//            image.set((x, y), &color).ok();

            for dx in 0..3 {
                if x == 0 || x == image.width()-1 { continue; }
                let x = x + dx - 1;
                for dy in 0..3 {
                    if y == 0 || y == image.height()-1 { continue; }
                    let y = y + dy - 1;

                    let previous = image.at((x, y));
                    if previous == black {

                        image.set((x, y), &color).ok();
                    } else {
                        image.set((x, y), &color.mix(200, &previous));
                    }
                }
            }
        }
    }
    image
}

fn main() {

    let c = Color::new(10, 0x10, 0xff);
    let white = Color::new(255, 255, 0);
    println!("{}", white.to_hsv().s);

    let camera = Camera {
        center: Complex::new(0.0, 0.0),
        height: 16.2832,
        screen_size: (780, 780),
    };

    let base_img = Image::load_png("base.png");

    let f = |z : Complex<f64>| (z).log(2.0);

    let maxi = 80;
    for i in 0..maxi {
        let prop = i as f64 / (maxi as f64 - 1.0);
        let g = |z| z * (1.0 - prop) + f(z) * prop;
        let mut img = transform(g, &camera, &base_img);

        // for i in 0..camera.number_of_pixels() {
        //     let coord = (i % camera.screen_size.0, i / camera.screen_size.0);
        //     let color = img.at(coord);
        //     let mut hsv = color.to_hsv();
        //     hsv.h += (prop * 360.0) as u16;
        //     hsv.h %= 360;
// //            hsv.s = (hsv.s + 100.0) / 2.0;
        //     img.set(coord, &hsv.to_rgb());
        // }
        img.save_png(&*format!("out/out{:03}.jpg", i));
    }
}
