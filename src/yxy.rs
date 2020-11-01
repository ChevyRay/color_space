use crate::{approx, FromRgb, Rgb, ToRgb, Xyz};

/// A CIE YXY color.
#[derive(Copy, Clone, Debug, Default)]
pub struct Yxy {
    pub y1: f64,
    pub x: f64,
    pub y2: f64,
}

impl Yxy {
    #[inline]
    pub fn new(y1: f64, x: f64, y2: f64) -> Self {
        Self { y1, x, y2 }
    }
}

impl PartialEq for Yxy {
    fn eq(&self, other: &Self) -> bool {
        approx(self.y1, other.y1) && approx(self.x, other.x) && approx(self.y2, other.y2)
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
            (1.0 - self.x - self.y2) * (self.y1 / self.y2),
        )
        .to_rgb()
    }
}
