use crate::{ FromRgb, ToRgb, approx };
use crate::rgb::Rgb;

/// A CIE 1931 XYZ color.
#[derive(Copy, Clone, Debug, Default)]
pub struct Xyz {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Xyz {
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl PartialEq for Xyz {
    fn eq(&self, other: &Self) -> bool {
        approx(self.x, other.x) &&
        approx(self.y, other.y) &&
        approx(self.z, other.z)
    }
}

fn srgb_to_linear(val: f64) -> f64 {
    if val <= 0.04045 {
        val / 12.92
    } else {
        ((val + 0.055) / 1.055).powf(2.4)
    }
}

impl FromRgb for Xyz {
    fn from_rgb(rgb: &Rgb) -> Self {
        let r = srgb_to_linear(rgb.r / 255.0);
        let g = srgb_to_linear(rgb.g / 255.0);
        let b = srgb_to_linear(rgb.b / 255.0);
        Self::new(
            (0.4124 * r + 0.3576 * g + 0.1805 * b) * 100.0,
            (0.2126 * r + 0.7152 * g + 0.0722 * b) * 100.0,
            (0.0193 * r + 0.1192 * g + 0.9505 * b) * 100.0
        )
        /*let r = rgb.r / 255.0;
        let g = rgb.g / 255.0;
        let b = rgb.b / 255.0;
        let r = 100.0 * if r > 0.04045 { ((r + 0.055) / 1.055).powf(2.4) } else { r / 12.92 };
        let g = 100.0 * if g > 0.04045 { ((g + 0.055) / 1.055).powf(2.4) } else { g / 12.92 };
        let b = 100.0 * if b > 0.04045 { ((b + 0.055) / 1.055).powf(2.4) } else { b / 12.92 };
        Self::new(
            r * 0.4124564 + g * 0.3575761 + b * 0.1804375,
            r * 0.2126729 + g * 0.7151522 + b * 0.0721750,
            r * 0.0193339 + g * 0.1191920 + b * 0.9503041
        )*/
    }
}

impl ToRgb for Xyz {
    fn to_rgb(&self) -> Rgb {
        let x = self.x / 100.0;
        let y = self.y / 100.0;
        let z = self.z / 100.0;
        let r = x * 3.2404542 + y * -1.5371385 + z * -0.4985314;
        let g = x * -0.9692660 + y * 1.8760108 + z * 0.0415560;
        let b = x * 0.0556434 + y * -0.2040259 + z * 1.0572252;
        let r = 255.0 * if r > 0.0031308 { 1.055 * r.powf(1.0 / 2.4) - 0.055 } else { 12.92 * r };
        let g = 255.0 * if g > 0.0031308 { 1.055 * g.powf(1.0 / 2.4) - 0.055 } else { 12.92 * g };
        let b = 255.0 * if b > 0.0031308 { 1.055 * b.powf(1.0 / 2.4) - 0.055 } else { 12.92 * b };
        Rgb::new(r, g, b)
    }
}