use crate::{ Rgb, FromRgb, ToRgb, Xyz, approx };

/// A CIELAB color.
#[derive(Copy, Clone, Debug, Default)]
pub struct Lab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

impl Lab {
    /// Create a new CIELAB color.
    /// 
    /// `l`: lightness component (0 to 100)
    /// 
    /// `a`: green (negative) and red (positive) component.
    /// 
    /// `b`: blue (negative) and yellow (positive) component.
    #[inline]
    pub const fn new(l: f64, a: f64, b: f64) -> Self {
        Self { l, a, b }
    }
}

impl PartialEq for Lab {
    fn eq(&self, other: &Self) -> bool {
        approx(self.l, other.l) &&
        approx(self.a, other.a) &&
        approx(self.b, other.b)
    }
}

impl FromRgb for Lab {
    fn from_rgb(rgb: &Rgb) -> Self {
        let xyz = Xyz::from_rgb(rgb);
        let x = xyz.x / 95.047;
        let y = xyz.y / 100.0;
        let z = xyz.z / 108.883;
        let x = if x > 0.008856 { x.cbrt() } else { 7.787 * x + 16.0 / 116.0 };
        let y = if y > 0.008856 { y.cbrt() } else { 7.787 * y + 16.0 / 116.0 };
        let z = if z > 0.008856 { z.cbrt() } else { 7.787 * z + 16.0 / 116.0 };
        Self::new(
            (116.0 * y) - 16.0,
            500.0 * (x - y),
            200.0 * (y - z)
        )
    }
}

impl ToRgb for Lab {
    fn to_rgb(&self) -> Rgb {
        let y = (self.l + 16.0) / 116.0;
		let x = self.a / 500.0 + y;
        let z = y - self.b / 200.0;
        let x3 = x.powf(3.0);
        let y3 = y.powf(3.0);
        let z3 = z.powf(3.0);
        let x = 95.047 * if x3 > 0.008856 { x3 } else { (x - 16.0 / 116.0) / 7.787 };
        let y = 100.0 * if y3 > 0.008856 { y3 } else { (y - 16.0 / 116.0) / 7.787 };
        let z = 108.883 * if z3 > 0.008856 { z3 } else { (z - 16.0 / 116.0) / 7.787 };
        Xyz::new(x, y, z).to_rgb()
    }
}