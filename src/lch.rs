use crate::{ Rgb, FromRgb, ToRgb, Lab };
use std::f64::consts::PI;

#[derive(Copy, Clone, Debug, Default)]
pub struct Lch {
    pub l: f64,
    pub c: f64,
    pub h: f64,
}

impl Lch {
    pub fn new(l: f64, c: f64, h: f64) -> Self {
        Self { l, c, h }
    }
}

impl FromRgb for Lch {
    fn from_rgb(rgb: &Rgb) -> Self {
        let lab = Lab::from_rgb(rgb);
        let l = lab.l;
        let c = (lab.a * lab.a + lab.b * lab.b).sqrt();
        let h = (lab.b.atan2(lab.a) / PI) * 180.0;
        Self::new(l, c, (h + 360.0) % 360.0)
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