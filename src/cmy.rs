use crate::{ Rgb, FromRgb, ToRgb, approx };

/// A CMY color (cyan, magenta, yellow).
#[derive(Copy, Clone, Debug, Default)]
pub struct Cmy {
    pub c: f64,
    pub m: f64,
    pub y: f64,
}

impl Cmy {
    /// Create a new CYM color.
    /// 
    /// `c`: cyan component (0 to 1)
    /// 
    /// `m`: magenta component (0 to 1)
    /// 
    /// `y`: yellow component (0 to 1)
    #[inline]
    pub fn new(c: f64, m: f64, y: f64) -> Self {
        Self { c, m, y }
    }
}

impl PartialEq for Cmy {
    fn eq(&self, other: &Self) -> bool {
        approx(self.c, other.c) &&
        approx(self.m, other.m) &&
        approx(self.y, other.y)
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