use super::tuple::{color, point, vector, Tup};
use raylib::prelude::*;
use std::convert::TryInto;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub trait Canvas {
    fn pixels(&mut self) -> &mut Vec<Pixel>;
    fn write_pixel(&mut self, x: u32, y: u32, color: Tup);
    fn pixel_at(&self, x: u32, y: u32) -> Tup;
}

pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub p: Tup,
}

pub struct PPMCanvas {
    pub pixels: Vec<Pixel>,
    pub height: u32,
    pub width: u32,
}

impl PPMCanvas {
    pub fn new(height: u32, width: u32) -> Self {
        let mut pixels = vec![];

        for x in 0..width {
            for y in 0..height {
                pixels.push(Pixel {
                    x,
                    y,
                    p: color(0.0, 0.0, 0.0),
                })
            }
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
            for num in &self.scale(&pixel.p) {
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
    fn pixels(&mut self) -> &mut Vec<Pixel> {
        &mut self.pixels
    }

    fn write_pixel(&mut self, x: u32, y: u32, color: Tup) {
        let offset = self.offset(x, y);
        let l = (self.height * self.width) - 1;
        if offset as u32 > l {
            return;
        }
        self.pixels[offset] = Pixel { x, y, p: color };
    }

    fn pixel_at(&self, x: u32, y: u32) -> Tup {
        // yolo? should be algebraic type
        return self.pixels[self.offset(x, y)].p.clone();
    }
}

pub struct OpenGLCanvas {
    pub pixels: Vec<Pixel>,
    pub height: u32,
    pub width: u32,
    pub title: String,
}

impl OpenGLCanvas {
    pub fn new(height: u32, width: u32, title: String) -> Self {
        let mut pixels = vec![];

        for x in 0..width {
            for y in 0..height {
                pixels.push(Pixel {
                    x,
                    y,
                    p: color(0.0, 0.0, 0.0),
                })
            }
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

    pub fn run(&mut self, rx: Receiver<Pixel>) {
        let (mut rl, thread) = raylib::init()
            .size(self.width as i32, self.height as i32)
            .title(&self.title)
            .build();

        while !rl.window_should_close() {
            // Get as many pixels as possible.
            loop {
                match rx.try_recv() {
                    Ok(pixel) => {
                        self.write_pixel(pixel.x, pixel.y, pixel.p);
                    },
                    Err(_) => break,
                }
            }

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
    fn pixels(&mut self) -> &mut Vec<Pixel> {
        &mut self.pixels
    }

    fn write_pixel(&mut self, x: u32, y: u32, color: Tup) {
        let offset = self.offset(x, y);
        let l = (self.height * self.width) - 1;
        if offset as u32 > l {
            return;
        }
        self.pixels[offset] = Pixel { x, y, p: color };
    }

    fn pixel_at(&self, x: u32, y: u32) -> Tup {
        // yolo? should be algebraic type
        return self.pixels[self.offset(x, y)].p.clone();
    }
}
