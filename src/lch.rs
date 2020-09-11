use crate::{ Rgb, FromRgb, ToRgb, Lab, approx };
use std::f64::consts::PI;

/// An LCH color (luminance, chroma, hue).
#[derive(Copy, Clone, Debug, Default)]
pub struct Lch {
    pub l: f64,
    pub c: f64,
    pub h: f64,
}

impl Lch {
    /// Create a new CIELAB color.
    /// 
    /// `l`: luminance component (0 to 100).
    /// 
    /// `c`: chroma component (0 to 100).
    /// 
    /// `h`: hue component (0 to 360).
    pub fn new(l: f64, c: f64, h: f64) -> Self {
        Self { l, c, h }
    }
}

impl PartialEq for Lch {
    fn eq(&self, other: &Self) -> bool {
        approx(self.l, other.l) &&
        approx(self.c, other.c) &&
        approx(self.h, other.h)
    }
}

impl FromRgb for Lch {
    fn from_rgb(rgb: &Rgb) -> Self {
        let lab = Lab::from_rgb(rgb);
        let c = (lab.a * lab.a + lab.b * lab.b).sqrt();
        let h = lab.b.atan2(lab.a) * (180.0 / PI);
        Self::new(lab.l, c, (h + 360.0) % 360.0)
    }
}

impl ToRgb for Lch {
    fn to_rgb(&self) -> Rgb {
        let h = (self.h * PI) / 180.0;
        Lab::new(
            self.l,
            h.cos() * self.c,
            h.sin() * self.c
        ).to_rgb()
    }
}