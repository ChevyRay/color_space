use crate::{ FromRgb, ToRgb };
use crate::rgb::Rgb;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Xyz {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Xyz {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

fn xyz_comp(comp: f64) -> f64 {
    let n = comp / 255.0;
    let res = match n > 0.04045 {
        true => ((n + 0.055) / 1.055).powf(2.4),
        false => n / 12.92,
    };
    res * 100.0
}

impl FromRgb for Xyz {
    fn from_rgb(rgb: &Rgb) -> Self {
        let r = xyz_comp(rgb.r);
        let g = xyz_comp(rgb.g);
        let b = xyz_comp(rgb.b);
        Self::new(
            r * 0.4124564 + g * 0.3575761 + b * 0.1804375,
            r * 0.2126729 + g * 0.7151522 + b * 0.0721750,
            r * 0.0193339 + g * 0.1191920 + b * 0.9503041
        )
    }
}

fn rgb_comp(comp: f64) -> f64 {
    let res = match comp > 0.0031308 {
        true => 1.055 * comp.powf(1.0 / 2.4) - 0.055,
        false => 12.92 * comp
    };
    res * 255.0
}

impl ToRgb for Xyz {
    fn to_rgb(&self) -> Rgb {
        let x = self.x / 100.0;
        let y = self.y / 100.0;
        let z = self.z / 100.0;
        let r = x * 3.2404542 + y * -1.5371385 + z * -0.4985314;
        let g = x * -0.9692660 + y * 1.8760108 + z * 0.0415560;
        let b = x * 0.0556434 + y * -0.2040259 + z * 1.0572252;
        Rgb::new(
            rgb_comp(r),
            rgb_comp(g),
            rgb_comp(b)
        )
    }
}