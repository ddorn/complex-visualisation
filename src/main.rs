extern crate num;

use num::complex::Complex;


type RPoint = Complex<f64>;
type SPoint = (usize, usize);
type Color = (u8, u8, u8);
type Image = Vec<Color>;


fn print_ppm(image : Vec<Color>, size : (usize, usize)) {
    assert_eq!(image.len(), size.0 * size.1);

    // ppm header
    println!("P3");  // specify RGB format
    println!("{} {}", size.0, size.1);  // image dims
    println!("255");  // color range from 0..256

    for y in 0..size.1 {
        for x in 0..size.0 {
            let color = image[y * size.0 + x];
            println!("{} {} {}", color.0, color.1, color.2);
        }
    }
}


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

    fn to_screen(&self, rp: RPoint) -> SPoint {
        let centered_x = (rp.re * (self.screen_size.0 as f64) / self.width()).round() as i32;
        let centered_y = (rp.im * (self.screen_size.1 as f64) / self.height).round() as i32;

        let x = (centered_x + self.screen_size.0 as i32 / 2) as usize;
        let y = (centered_y + self.screen_size.1 as i32 / 2) as usize;

        (x, self.screen_size.1 - y)
    }

    fn number_of_pixels(&self) -> usize {
        self.screen_size.0 * self.screen_size.1
    }

    fn real_to_color(&self, z: RPoint) -> Color {
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
}


fn main() {
    let camera = Camera {
        center: Complex::default(),
        height: 2.0,
        screen_size: (720, 720),
    };

    let image : Image =
        (0..camera.number_of_pixels())
        .map(|i| (i % camera.screen_size.0, i / camera.screen_size.0))
        .map(|sp| camera.to_real(sp))
        .map(|z| 1.0 / (1.0 - z))
        .map(|z| camera.real_to_color(z))
        .collect();

    print_ppm(image, camera.screen_size);

}
