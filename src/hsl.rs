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
    /// `s`: saturation component (0 to 1)
    ///
    /// `l`: light component (0 to 1)
    #[inline]
    pub const fn new(h: f64, s: f64, l: f64) -> Self {
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
        let red = rgb.r / 255.0;
        let green = rgb.g / 255.0;
        let blue = rgb.b / 255.0;
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

impl ToRgb for Hsl {
    fn to_rgb(&self) -> Rgb {
        let (hue, saturation, lightness) = (self.h, self.s, self.l);

        let mut chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation * 255.0;
        let mut x = chroma * (1.0 - (hue / 60.0 % 2.0 - 1.0).abs());
        let min = lightness * 255.0 - chroma / 2.0;
        chroma += min;
        x += min;

        let (red, green, blue) = if hue <= 60.0 {
            (chroma, x, min)
        } else if hue <= 120.0 {
            (x, chroma, min)
        } else if hue <= 180.0 {
            (min, chroma, x)
        } else if hue <= 240.0 {
            (min, x, chroma)
        } else if hue <= 300.0 {
            (x, min, chroma)
        } else {
            (chroma, min, x)
        };

        Rgb::new(red, green, blue)
    }
}
