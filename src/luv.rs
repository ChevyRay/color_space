use crate::{ Rgb, FromRgb, ToRgb, Xyz };

#[derive(Copy, Clone, Debug, Default)]
pub struct Luv {
    pub l: f64,
    pub u: f64,
    pub v: f64,
}

impl Luv {
    pub fn new(l: f64, u: f64, v: f64) -> Self {
        Self { l, u, v }
    }
}

const EPS: f64 = 216.0 / 24389.0;
const KAPPA: f64 = 24389.0 / 27.0;
const WHITE: Xyz = Xyz{ x: 95.047, y: 100.000, z: 108.883 };

impl FromRgb for Luv {
    fn from_rgb(rgb: &Rgb) -> Self {

        let xyz = Xyz::from_rgb(rgb);
        let y = xyz.y / WHITE.y;
        let temp = xyz.x + 15.0 * xyz.y + 3.0 * xyz.z;
        let tempr = WHITE.x + 15.0 * WHITE.y + 3.0 * WHITE.z;
        
        let l = match y > EPS {
            true => 116.0 * y.cbrt() - 16.0,
            false => KAPPA * y
        };

        let (u, v) = match temp > 1e-3 {
            true => (xyz.x / temp, xyz.y / temp),
            false => (0.0, 0.0),
        };

        Self::new(
            l,
            52.0 * l * (u - WHITE.x / tempr),
            117.0 * l * (v - WHITE.y / tempr)
        )
    }
}

impl ToRgb for Luv {
    fn to_rgb(&self) -> Rgb {
        let y = match self.l > EPS * KAPPA {
            true => ((self.l + 16.0) / 116.0).powf(3.0),
            false => self.l / KAPPA,
        };
        let tempr = WHITE.x + 15.0 * WHITE.y + 3.0 * WHITE.z;
        let up = 4.0 * WHITE.x / tempr;
        let vp = 9.0 * WHITE.y / tempr;
        let a = 1.0 / 3.0 * (52.0 * self.l / (self.u + 13.0 * self.l * up) - 1.0);
        let b = y * -5.0;
        let x = (y * (39.0 * self.l / (self.v + 13.0 * self.l * vp) - 5.0) - b) / (a + 1.0 / 3.0);
        let z = x * a + b;
        Xyz::new(x * 100.0, y * 100.0, z * 100.0).to_rgb()
    }
}