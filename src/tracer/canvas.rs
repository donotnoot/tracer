use super::tuple::{color, Tup};
use raylib::color::Color;
use raylib::prelude::*;
use std::convert::TryInto;

use std::sync::mpsc::Receiver;

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
        fn s(f: f32) -> u8 {
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
        self.pixels[self.offset(x, y)].p.clone()
    }
}

pub struct OpenGLCanvas {
    pub pixels: Vec<Pixel>,
    pub height: u32,
    pub width: u32,
    pub title: String,
}

impl OpenGLCanvas {
    pub fn new(width: u32, height: u32, title: String, background_color: Tup) -> Self {
        let mut pixels = vec![];

        for x in 0..width {
            for y in 0..height {
                pixels.push(Pixel {
                    x,
                    y,
                    p: background_color.clone(),
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
        fn s(f: f32) -> u8 {
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

        let mut show_hud = false;
        let total_pixels = self.width * self.height;
        let mut update_counter = 60;
        let mut pixels_received: i64 = 0;
        let mut pixels_s: i32 = 0;
        let mut eta: i32 = 0;

        let stats_bg_color = Color::new(20, 20, 20, 230);
        let stats_fg_color = Color::new(255, 255, 255, 255);

        while !rl.window_should_close() {
            // Get as many pixels as possible.
            while let Ok(pixel) = rx.try_recv() {
                self.write_pixel(pixel.x, pixel.y, pixel.p);
                pixels_received += 1;
            }

            if rl.is_key_pressed(KeyboardKey::KEY_S) {
                show_hud = !show_hud;
            }

            if update_counter == 60 {
                let t = rl.get_time() as i64;
                pixels_s = if t != 0 {
                    (pixels_received / t) as i32
                } else {
                    pixels_received as i32
                };
                eta = (total_pixels as i64 - pixels_received) as i32 / pixels_s;
                update_counter = 0;
            }
            update_counter += 1;

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

            if show_hud {
                let progress_ratio = pixels_received as f32 / total_pixels as f32;
                let progress_percent = progress_ratio * 100.;

                d.draw_rectangle(
                    5,
                    5,
                    5 + ((self.width - 15) as f32 * progress_ratio) as i32,
                    10,
                    raylib::color::Color::BLUE,
                );
                d.draw_rectangle(5, 15, 200, 100, &stats_bg_color);
                d.draw_text(
                    &format!(
                        "progress: {:.1}%\npixels/s: {}\nETA: {}s",
                        progress_percent, pixels_s, eta
                    )
                    .to_owned(),
                    10,
                    20,
                    20,
                    &stats_fg_color,
                );
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
        self.pixels[self.offset(x, y)].p.clone()
    }
}
