use super::tuple::{color, point, vector, Tup};
use raylib::prelude::*;
use std::convert::TryInto;

pub trait Canvas {
    fn write_pixel(&mut self, x: u32, y: u32, color: Tup);
    fn pixel_at(&self, x: u32, y: u32) -> Tup;
}

pub struct PPMCanvas {
    pub pixels: Vec<Tup>,
    pub height: u32,
    pub width: u32,
}

impl PPMCanvas {
    pub fn new(height: u32, width: u32) -> Self {
        let mut pixels = vec![];

        for _ in 0..height * width {
            pixels.push(color(0.0, 0.0, 0.0))
        }

        PPMCanvas {
            pixels,
            height,
            width,
        }
    }

    fn offset(&self, x: u32, y: u32) -> usize {
        (y * self.width + x).try_into().unwrap()
    }

    fn scale(&self, p: &Tup) -> [u8; 3] {
        fn s(f: f64) -> u8 {
            let n = (255.0 * f).ceil() as i16;
            if n > 255 {
                return 255 as u8;
            }
            if n < 0 {
                return 0 as u8;
            }
            n as u8
        }
        [s(p.x), s(p.y), s(p.z)]
    }

    pub fn ppm_stdout(&self) {
        print!("P3\n{} {}\n255\n", self.width, self.height);

        fn intlen(i: u8) -> u8 {
            match i {
                0..=9 => 1,
                10..=99 => 2,
                _ => 3,
            }
        }
        let mut chars_written_to_line = 0;
        let mut ints_written = 0;

        for pixel in &self.pixels {
            for num in &self.scale(pixel) {
                let l = intlen(*num);

                let line_len_after_write = chars_written_to_line + l + 1;
                if line_len_after_write > 70 {
                    chars_written_to_line = 0;
                    println!();
                } else if chars_written_to_line != 0 {
                    print!(" ");
                    chars_written_to_line += 1;
                }

                print!("{}", num);
                chars_written_to_line += l;
                ints_written += 1;

                if ints_written / 3 == self.width {
                    ints_written = 0;
                    chars_written_to_line = 0;
                    println!();
                }
            }
        }

        println!();
    }
}

impl Canvas for PPMCanvas {
    fn write_pixel(&mut self, x: u32, y: u32, color: Tup) {
        let offset = self.offset(x, y);
        let l = (self.height * self.width) - 1;
        if offset as u32 > l {
            return;
        }
        self.pixels[offset] = color;
    }

    fn pixel_at(&self, x: u32, y: u32) -> Tup {
        // yolo? should be algebraic type
        return self.pixels[self.offset(x, y)].clone();
    }
}

pub struct OpenGLCanvas {
    pub pixels: Vec<Tup>,
    pub height: u32,
    pub width: u32,
    pub title: String,
}

impl OpenGLCanvas {
    pub fn new(height: u32, width: u32, title: String) -> Self {
        let mut pixels = vec![];

        for _ in 0..height * width {
            pixels.push(color(0.0, 0.0, 0.0))
        }

        OpenGLCanvas {
            pixels,
            height,
            width,
            title,
        }
    }

    fn offset(&self, x: u32, y: u32) -> usize {
        (y * self.width + x).try_into().unwrap()
    }

    fn tup_to_rl_color(&self, t: Tup) -> raylib::color::Color {
        fn s(f: f64) -> u8 {
            let n = (255.0 * f).ceil() as i16;
            if n > 255 {
                return 255 as u8;
            }
            if n < 0 {
                return 0 as u8;
            }
            n as u8
        }
        raylib::color::Color {
            r: s(t.x),
            g: s(t.y),
            b: s(t.z),
            a: 255,
        }
    }

    pub fn run(&self) {
        let (mut rl, thread) = raylib::init().size(self.width as i32, self.height as i32).title(&self.title).build();

        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);

            for x in 0..self.width as i32 {
                for y in 0..self.height as i32 {
                    d.draw_pixel(
                        x,
                        y,
                        self.tup_to_rl_color(self.pixel_at(x as u32, y as u32)),
                    )
                }
            }
        }
    }
}

impl Canvas for OpenGLCanvas {
    fn write_pixel(&mut self, x: u32, y: u32, color: Tup) {
        let offset = self.offset(x, y);
        let l = (self.height * self.width) - 1;
        if offset as u32 > l {
            return;
        }
        self.pixels[offset] = color;
    }

    fn pixel_at(&self, x: u32, y: u32) -> Tup {
        // yolo? should be algebraic type
        return self.pixels[self.offset(x, y)].clone();
    }
}
