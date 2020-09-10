use crate::{ Rgb, FromRgb, ToRgb };

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Cmy {
    pub c: f64,
    pub m: f64,
    pub y: f64,
}

impl Cmy {
    pub fn new(c: f64, m: f64, y: f64) -> Self {
        Self { c, m, y }
    }
}

impl FromRgb for Cmy {
    fn from_rgb(rgb: &Rgb) -> Self {
        Self::new(
            1.0 - rgb.r / 255.0,
            1.0 - rgb.g / 255.0,
            1.0 - rgb.b / 255.0
        )
    }
}

impl ToRgb for Cmy {
    fn to_rgb(&self) -> Rgb {
        Rgb::new(
            (1.0 - self.c) * 255.0,
            (1.0 - self.m) * 255.0,
            (1.0 - self.y) * 255.0
        )
    }
}