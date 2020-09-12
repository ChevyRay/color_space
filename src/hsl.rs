use crate::{approx, FromRgb, Rgb, ToRgb};

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
    #[inline]
    pub fn new(h: f64, s: f64, l: f64) -> Self {
        Self { h, s, l }
    }
}

impl PartialEq for Hsl {
    fn eq(&self, other: &Self) -> bool {
        approx(self.h, other.h) && approx(self.s, other.s) && approx(self.l, other.l)
    }
}

impl FromRgb for Hsl {
    fn from_rgb(rgb: &Rgb) -> Self {
        let (red, green, blue) = (rgb.r, rgb.g, rgb.b);
        let min = red.min(green).min(blue);
        let max = red.max(green).max(blue);
        let chroma = max - min;
        let lightness = (max + min) / 2.0;

        let hue = if chroma == 0.0 {
            0.0
        } else if max == red {
            (green - blue) / chroma % 6.0
        } else if max == green {
            (blue - red) / chroma + 2.0
        } else {
            (red - green) / chroma + 4.0
        } * 60.0;

        let saturation = if chroma == 0.0 || lightness == 0.0 || lightness == 1.0 {
            0.0
        } else {
            (max - lightness) / lightness.min(1.0 - lightness)
        };

        Self::new(hue, saturation, lightness)
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
        let l = self.l;
        match self.s == 0.0 {
            true => Rgb::new(l * 255.0, l * 255.0, l * 255.0),
            false => {
                let h = self.h / 360.0;
                let s = self.s;

                let temp2 = if l < 0.5 {
                    l * (1.0 + s)
                } else {
                    l + s - (s * l)
                };
                let temp1 = 2.0 * l - temp2;
                Rgb::new(
                    255.0 * hue_to_rgb(temp1, temp2, h + 1.0 / 3.0),
                    255.0 * hue_to_rgb(temp1, temp2, h),
                    255.0 * hue_to_rgb(temp1, temp2, h - 1.0 / 3.0),
                )
            }
        }
    }
}
