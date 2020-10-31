use crate::{approx, FromRgb, Rgb, ToRgb};

/// An HSV color (hue, saturation, value).
#[derive(Copy, Clone, Debug, Default)]
pub struct Hsv {
    pub h: f64,
    pub s: f64,
    pub v: f64,
}

impl Hsv {
    /// Create a new HSV color.
    ///
    /// `h`: hue component (0 to 360)
    ///
    /// `s`: saturation component (0 to 1)
    ///
    /// `v`: value component (0 to 1)
    #[inline]
    pub fn new(h: f64, s: f64, v: f64) -> Self {
        Self { h, s, v }
    }
}

impl PartialEq for Hsv {
    fn eq(&self, other: &Self) -> bool {
        approx(self.h, other.h) && approx(self.s, other.s) && approx(self.v, other.v)
    }
}

impl FromRgb for Hsv {
    fn from_rgb(rgb: &Rgb) -> Self {
        let r = rgb.r / 255.0;
        let g = rgb.g / 255.0;
        let b = rgb.b / 255.0;

        let min = r.min(g.min(b));
        let max = r.max(g.max(b));
        let delta = max - min;

        let v = max;
        let s = match max > 1e-3 {
            true => delta / max,
            false => 0.0,
        };
        let h = match delta == 0.0 {
            true => 0.0,
            false => {
                if r == max {
                    (g - b) / delta
                } else if g == max {
                    2.0 + (b - r) / delta
                } else {
                    4.0 + (r - g) / delta
                }
            }
        };
        let h2 = ((h * 60.0) + 360.0) % 360.0;

        Self::new(h2, s, v)
    }
}

impl ToRgb for Hsv {
    fn to_rgb(&self) -> Rgb {
        let range = (self.h / 60.0) as u8;
        let c = self.v * self.s;
        let x = c * (1.0 - (((self.h / 60.0) % 2.0) - 1.0).abs());
        let m = self.v - c;

        match range {
            0 => Rgb::new((c + m) * 255.0, (x + m) * 255.0, m * 255.0),
            1 => Rgb::new((x + m) * 255.0, (c + m) * 255.0, m * 255.0),
            2 => Rgb::new(m * 255.0, (c + m) * 255.0, (x + m) * 255.0),
            3 => Rgb::new(m * 255.0, (x + m) * 255.0, (c + m) * 255.0),
            4 => Rgb::new((x + m) * 255.0, m * 255.0, (c + m) * 255.0),
            _ => Rgb::new((c + m) * 255.0, m * 255.0, (x + m) * 255.0),
        }
    }
}
