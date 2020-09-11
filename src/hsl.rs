use crate::{ Rgb, FromRgb, ToRgb, approx };

/// An HSL color (hue, saturation, light).
#[derive(Copy, Clone, Debug, Default)]
pub struct Hsl {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

impl Hsl {
    /// Create a new HSL color.
    /// 
    /// `h`: hue component (0 to 360)
    /// 
    /// `s`: saturation component (0 to 100)
    /// 
    /// `l`: light component (0 to 100)
    pub fn new(h: f64, s: f64, l: f64) -> Self {
        Self { h, s, l }
    }
}

impl PartialEq for Hsl {
    fn eq(&self, other: &Self) -> bool {
        approx(self.h, other.h) &&
        approx(self.s, other.s) &&
        approx(self.l, other.l)
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
        match delta == 0.0 {
            true => Self::new(0.0, 0.0, l * 100.0),
            false => {
                let s = match l < 0.5 {
                    true => delta / (max + min),
                    false => delta / (1.0 - (2.0 * l - 1.0).abs()),
                };
                let h = if r == max {
                    (g - b) / delta
                } else if g == max {
                    (b - r) / delta + 2.0
                } else {
                    (r - g) / delta + 4.0
                };
                Self::new(
                    (h * 60.0 + 360.0) % 360.0,
                    s * 100.0,
                    l * 100.0
                )
            }
        }
    }
}

fn hue_to_rgb(v1: f64, v2: f64, vh: f64) -> f64 {
    let vh = (vh + 1.0) % 1.0;
    if 6.0 * vh < 1.0 {
        v1 + (v2 - v1) * 6.0 * vh
    } else if 2.0 * vh < 1.0 {
        v2
    } else if 3.0 * vh < 2.0 {
        v1 + (v2 - v1) * (2.0 / 3.0 - vh) * 6.0
    } else {
        v1
    }
}

impl ToRgb for Hsl {
    fn to_rgb(&self) -> Rgb {
        let l = self.l / 100.0;
        match self.s == 0.0 {
            true => {
                Rgb::new(
                    l * 255.0,
                    l * 255.0,
                    l * 255.0
                )
            }
            false => {
                let h = self.h / 360.0;
                let s = self.s / 100.0;
                
                let temp2 = if l < 0.5 { l * (1.0 + s) } else { l + s - (s * l) };
                let temp1 = 2.0 * l - temp2;
                Rgb::new(
                    255.0 * hue_to_rgb(temp1, temp2, h + 1.0 / 3.0),
                    255.0 * hue_to_rgb(temp1, temp2, h),
                    255.0 * hue_to_rgb(temp1, temp2, h - 1.0 / 3.0)
                )
            }
        }
    }
}