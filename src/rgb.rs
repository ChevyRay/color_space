use crate::{ FromRgb, ToRgb };

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Rgb {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
    pub fn hex(hex: u32) -> Self {
        Self {
            r: (((hex >> 16) & 0xff) as f64) / 255.0,
            g: (((hex >> 8) & 0xff) as f64) / 255.0,
            b: ((hex & 0xff) as f64) / 255.0,
        }
    }
}

impl FromRgb for Rgb {
    fn from_rgb(rgb: &Rgb) -> Self {
        *rgb
    }
}

impl ToRgb for Rgb {
    fn to_rgb(&self) -> Rgb {
        *self
    }
}