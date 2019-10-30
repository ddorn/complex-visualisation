extern crate raster;
extern crate png;

// For reading and opening files
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
// To use encoder.set()
use png::Encoder;
use raster::Color as Kevin;

#[derive(Eq, PartialEq)]
pub struct Color{
    color: u32
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { color: (r as u32) << 16 | (g as u32) << 8 | b as u32  }
    }
    pub fn mix(&self, t: u8, other: &Color) -> Color {
        let nt = (!t) as u32;
        let t = t as u32;

        // mix R and B
        let rb = self.color & 0xff00ff;
        let rb2 = other.color & 0xff00ff;
        let rb = (rb * nt + rb2 * t) >> 8 & 0xff00ff;

        // mix green
        let g = self.color & 0xff00;
        let g2 = other.color & 0xff00;
//        println!("{:x} {:x} {:x}", g, g * nt, g*nt >> 8);
        let g = (g * nt + g2 * t) >> 8 & 0xff00;
        Color { color: rb | g}
    }

    pub fn to_hsv(&self) -> HSV {
        let (h, s, v) = Kevin::to_hsv(self.r(), self.g(), self.b());
        HSV {
            h, s, v
        }
    }

    pub fn r(&self) -> u8 { (self.color >> 16) as u8 }
    pub fn g(&self) -> u8 { (self.color >> 8) as u8 }
    pub fn b(&self) -> u8 { self.color as u8 }

    pub fn show(&self) {
        println!("{:x}", self.color);
    }
}

pub struct HSV {
    pub h: u16,
    pub s: f32,
    pub v: f32
}

impl HSV {
    pub fn to_rgb(&self) -> Color {
        let (r, g, b) = Kevin::to_rgb(self.h, self.s, self.v);
        Color::new(r, g, b)
    }
}

pub struct Image {
    pixels : Vec<u8>,
    width : u32,
    height : u32,
    pixel_size : u32,
}

impl Image {
    pub fn new(width: u32, height: u32 ) -> Image {
        Image {
            pixels : vec![0; (width * height * 3) as usize],
            width,
            height,
            pixel_size: 3
        }
    }

    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }

    pub fn at(&self, (x, y) : (u32, u32)) -> Color {
        let start = ((y * self.width + x) * self.pixel_size) as usize;

        Color::new(
            self.pixels[start],
            self.pixels[start + 1],
            self.pixels[start + 2]
        )
    }

    pub fn set(&mut self, (x, y): (u32, u32), color: &Color) -> Result<(), (u32, u32)>{
        if x >= self.width || y >= self.height {
            Err((x, y))
        } else {
            let start = ((y * self.width + x) * self.pixel_size) as usize;

            self.pixels[start] = color.r();
            self.pixels[start + 1] = color.g();
            self.pixels[start + 2] = color.b();

            Ok(())
        }
    }

    pub fn load_png(path: &str) -> Image {
        // The decoder is a build for reader and can be used to set various decoding options
        // via `Transformations`. The default output transformation is `Transformations::EXPAND
        // | Transformations::STRIP_ALPHA`.
        let decoder = png::Decoder::new(File::open(path).unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();
        // Allocate the output buffer.
        let mut buf = vec![0; info.buffer_size()];
        // Read the next frame. Currently this function should only called once.
        // The default options
        reader.next_frame(&mut buf).unwrap();

        Image {
            pixels: buf,
            width: info.width as u32,
            height: info.height as u32,
            pixel_size: 3
        }
    }

    pub fn save_png(&self, path: &str) {

        let path = Path::new(path);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&self.pixels).unwrap();
    }

    pub fn print_ppm(&self) {
        // ppm header
        println!("P3");  // specify RGB format
        println!("{} {}", self.width, self.height);  // image dims
        println!("255");  // color range from 0..256

        for i in (0..self.width*self.height).map(|x| (x * self.pixel_size) as usize) {
            println!("{} {} {}", self.pixels[i], self.pixels[i+1], self.pixels[i+2]);
        }
    }
}

//
//impl Index<(u32, u32)> for Image {
//    typ// Check if source RGB is equal to final RGBe Output = Color;
//
//    fn index(&self, (x, y): (u32, u32)) -> Self::Output {
//        let start = (y * self.width + x) * 3;
//        Color::new(
//            self.pixels[start],
//            self.pixels[start + 1],
//            self.pixels[start + 2]
//        )
//    }
//}
/*
impl IndexMut<(u32, u32)> for Image {
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut (&mut u8, &mut u8, &mut u8) {
        if x >= self.width || y >= self.height {
            panic!(format!("Access of pixel at ({}, {}) when image is {}x{}", x, y, self.width, self.height));
        }

        let start = (y * self.width + x) * 3;

        (
            &mut self.pixels[start],
            &mut self.pixels[start + 1],
            &mut self.pixels[start + 2]
        )
    }
}
*/

