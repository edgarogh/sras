use minifb::{Scale, Window, WindowOptions};

#[derive(Clone, Copy, Default)]
pub struct Glyph(pub [u8; 5]);

impl std::fmt::Debug for Glyph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<glyph>")
    }
}

impl std::ops::Index<(usize, usize)> for Glyph {
    type Output = Color;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        debug_assert!(x < 5);
        debug_assert!(y < 8);

        let byte = self.0[x];

        if (byte >> y) & (0b00000001) != 0 {
            &Color::On
        } else {
            &Color::Off
        }
    }
}

include!("charset.cpp.rs");

pub struct PCD8544 {
    inner: Option<Box<PCD8544Window>>,
}

impl PCD8544 {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            inner: Some(Box::new(PCD8544Window::new(width, height))),
        }
    }

    pub fn create_char(&mut self, c: u8, glyph: Glyph) {
        if let Some(window) = &mut self.inner {
            window.create_char(c, glyph);
        }
    }

    pub fn update(&mut self) {
        if let Some(window) = &mut self.inner {
            window.window.update();
            if !window.window.is_open() {
                self.inner = None;
            }
        }
    }

    pub fn set_cursor(&mut self, x: usize, y: usize) {
        if let Some(window) = &mut self.inner {
            window.set_cursor(x, y);
        }
    }

    pub fn write(&mut self, char: u8) {
        if let Some(window) = &mut self.inner {
            window.write(char);
        }
    }

    pub fn clear(&mut self) {
        if let Some(window) = &mut self.inner {
            window.clear();
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum Color {
    Off = 0x3d3d0d,
    On = 0xa0af37,
}

const CHAR_WIDTH: usize = 5;
const CHAR_HEIGHT: usize = 8;

struct PCD8544Window {
    buffer: Vec<Color>,
    window: Window,
    window_size: (usize, usize),
    size: (usize, usize),
    cursor: (usize, usize),
    glyphs: [Glyph; 128],
}

impl PCD8544Window {
    pub fn new(width: usize, height: usize) -> Self {
        let pixel_width = width * (1 + CHAR_WIDTH) + 1;
        let pixel_height = height * (1 + CHAR_HEIGHT) + 1;

        PCD8544Window {
            buffer: vec![Color::Off; pixel_width * pixel_height],
            window: Window::new("PCD8544", pixel_width, pixel_height, {
                let mut opts = WindowOptions::default();
                opts.scale = Scale::X4;
                opts
            })
            .unwrap(),
            window_size: (pixel_width, pixel_height),
            size: (width, height),
            cursor: (0, 0),
            glyphs: DEFAULT_CHARSET,
        }
    }

    pub fn create_char(&mut self, c: u8, glyph: Glyph) {
        self.glyphs[c as usize] = glyph;
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if let Some(pixel) = self.buffer.get_mut(y * self.window_size.0 + x) {
            *pixel = color;
        } else {
            eprintln!(
                "Cannot set color on pixel ({}, {}) outside the display",
                x, y,
            )
        }
    }

    pub fn write(&mut self, c: u8) {
        let (x, y) = self.cursor;
        let (sx, sy) = (1 + x * (CHAR_WIDTH + 1), 1 + y * (CHAR_HEIGHT + 1));
        let glyph = self.glyphs[c as usize];

        for y in 0..CHAR_HEIGHT {
            for x in 0..CHAR_WIDTH {
                self.set_pixel(sx + x, sy + y, glyph[(x, y)]);
            }
        }

        self.update();

        self.set_cursor(x + 1, y);
    }

    pub fn set_cursor(&mut self, x: usize, y: usize) {
        self.cursor = (x % self.size.0, y % self.size.1);
    }

    pub fn clear(&mut self) {
        self.buffer.fill(Color::Off);
        self.update();
    }

    fn update(&mut self) {
        let buffer = unsafe { std::mem::transmute(self.buffer.as_slice()) };

        self.window
            .update_with_buffer(buffer, self.window_size.0, self.window_size.1)
            .unwrap()
    }
}
