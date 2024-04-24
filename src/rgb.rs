use crate::{ FromRgb, ToRgb, approx };

/// An RGB color (red, green, blue).
#[derive(Copy, Clone, Debug, Default)]
pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Rgb {
    /// Create a new RGB color.
    /// 
    /// `r`: red component (0 to 255).
    /// 
    /// `g`: green component (0 to 255).
    /// 
    /// `b`: blue component (0 to 255).
    #[inline]
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    /// Create a new RGB color from the `hex` value.
    /// 
    /// ```let cyan = Rgb::from_hex(0x00ffff);```
    pub const fn from_hex(hex: u32) -> Self {
        Self {
            r: (((hex >> 16) & 0xff) as f64),
            g: (((hex >> 8) & 0xff) as f64),
            b: ((hex & 0xff) as f64)
        }
    }
}

impl PartialEq for Rgb {
    fn eq(&self, other: &Self) -> bool {
        approx(self.r, other.r) &&
        approx(self.g, other.g) &&
        approx(self.b, other.b)
    }
}

impl FromRgb for Rgb {
    fn from_rgb(rgb: &Rgb) -> Self {
        *rgb
    }
}

impl ToRgb for Rgb {
    fn to_rgb(&self) -> Rgb {
        *self
    }
}