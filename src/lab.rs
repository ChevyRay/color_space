use crate::{ Rgb, FromRgb, ToRgb, Xyz };

#[derive(Copy, Clone, Debug, Default)]
pub struct Lab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

impl Lab {
    pub fn new(l: f64, a: f64, b: f64) -> Self {
        Self { l, a, b }
    }
}

fn rgb_to_lab(comp: f64) -> f64 {
    match comp > 0.008856 {
        true => comp.cbrt(),
        false => comp * 7.787 + 16.0 / 116.0,
    }
}

impl FromRgb for Lab {
    fn from_rgb(rgb: &Rgb) -> Self {
        let xyz = Xyz::from_rgb(rgb);
        let x = rgb_to_lab(xyz.x / 95.047);
        let y = rgb_to_lab(xyz.y / 100.00);
        let z = rgb_to_lab(xyz.z / 108.883);
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
        Xyz::new(
            (if x3 > 0.008856 { x3 } else { (x - 16.0 / 116.0) / 7.787 }) * 95.047,
            (if y3 > 0.008856 { y3 } else { (x - 16.0 / 116.0) / 7.787 }) * 100.0,
            (if z3 > 0.008856 { z3 } else { (x - 16.0 / 116.0) / 7.787 }) * 108.883,
        ).to_rgb()
    }
}