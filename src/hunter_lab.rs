use crate::{ Rgb, FromRgb, ToRgb, Xyz };

#[derive(Copy, Clone, Debug, Default)]
pub struct HunterLab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

impl HunterLab {
    pub fn new(l: f64, a: f64, b: f64) -> Self {
        Self { l, a, b }
    }
}

impl FromRgb for HunterLab {
    fn from_rgb(rgb: &Rgb) -> Self {
        let xyz = Xyz::from_rgb(rgb);
        let sqrt_y = xyz.y.sqrt();
        match xyz.y != 0.0 {
            true => Self::new(
                sqrt_y * 10.0,
                17.5 * (1.02 * xyz.x - xyz.y) / sqrt_y,
                7.0 * (xyz.y - 0.847 * xyz.z) / sqrt_y
            ),
            false => Self::new(
                sqrt_y * 10.0,
                0.0,
                0.0
            )
        }
    }
}

impl ToRgb for HunterLab {
    fn to_rgb(&self) -> Rgb {
        let x = (self.a / 17.5) * (self.l / 10.0);
        let y = self.l * self.l / 100.0;
        let z = self.b / 7.0 * self.l / 10.0;
        Xyz::new(
            (x + y) / 1.02,
            y,
            -(z - y) / 0.847
        ).to_rgb()
    }
}