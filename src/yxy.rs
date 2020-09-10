use crate::{ Rgb, FromRgb, ToRgb, Xyz };

#[derive(Copy, Clone, Debug, Default)]
pub struct Yxy {
    pub y1: f64,
    pub x: f64,
    pub y2: f64,
}

impl Yxy {
    pub fn new(y1: f64, x: f64, y2: f64) -> Self {
        Self { y1, x, y2 }
    }
}

impl FromRgb for Yxy {
    fn from_rgb(rgb: &Rgb) -> Self {
        let xyz = Xyz::from_rgb(rgb);
        let temp = xyz.x + xyz.y + xyz.z;
        match temp == 0.0 {
            true => Yxy::new(xyz.y, 0.0, 0.0),
            false => Yxy::new(xyz.y, xyz.x / temp, xyz.y / temp),
        }
    }
}

impl ToRgb for Yxy {
    fn to_rgb(&self) -> Rgb {
        Xyz::new(
            self.x * (self.y1 / self.y2),
            self.y1,
            (1.0 - self.x - self.y2) * (self.y1 / self.y2)
        ).to_rgb()
    }
}