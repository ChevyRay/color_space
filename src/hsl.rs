use crate::{ Rgb, FromRgb, ToRgb };

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Hsl {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

impl Hsl {
    pub fn new(h: f64, s: f64, l: f64) -> Self {
        Self { h, s, l }
    }
}

impl FromRgb for Hsl {
    fn from_rgb(rgb: &Rgb) -> Self {
        let r = rgb.r / 255.0;
        let g = rgb.g / 255.0;
        let b = rgb.b / 255.0;
        let min = r.min(g.min(b));
        let max = r.max(g.max(b));
        let delta = max - min;
        let l = (max + min) / 2.0;
        
        let (h, s) = match delta == 0.0 {
            true => (0.0, 0.0),
            false => {
                let s = match l < 0.5 {
                    true => delta / (max + min) * 100.0,
                    false => delta / (1.0 - (2.0 * l - 1.0).abs()) * 100.0,
                };
                let h = if r == max {
                    (g - b) / delta
                } else if g == max {
                    (b - r) / delta + 2.0
                } else {
                    (r - g) / delta + 4.0
                };
                let h2 = (60.0 * h + 360.0) % 360.0;
                (h2, s)
            },
        };

        Self::new(h, s, l * 100.0)
    }
}

fn hue_to_rgb(v1: f64, v2: f64, vh: f64) -> f64 {
    let mut v = vh;
    if v < 0.0 { v += 1.0; }
    if v > 1.0 { v -= 1.0 }
    if v * 6.0 < 1.0 {
        v1 + (v2 - v1) * 6.0 * v
    } else if v1 * 2.0 < 1.0 {
        v2
    } else if v * 3.0 < 2.0 {
        v1 + (v2 - v1) * (2.0 / 3.0 - v) * 6.0
    } else {
        v1
    }
}

impl ToRgb for Hsl {
    fn to_rgb(&self) -> Rgb {
        let h = self.h / 360.0;
        let s = self.s / 100.0;
        let l = self.l / 100.0;
        match s == 0.0 {
            true => {
                let val = l * 255.0;
                Rgb::new(val, val, val)
            },
            false => {
                let t2 = match l < 0.5 {
                    true => l * (1.0 + s),
                    false => l + s - (s * l),
                };
                let t1 = 2.0 * l - t2;
                Rgb::new(
                    hue_to_rgb(t1, t2, h + 1.0 / 3.0) * 255.0,
                    hue_to_rgb(t1, t2, h) * 255.0,
                    hue_to_rgb(t1, t2, h - 1.0 / 3.0) * 255.0
                )
            }
        }
    }
}