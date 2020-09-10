mod cmy;
mod cmyk;
mod hsl;
mod hsv;
mod hunter_lab;
mod lab;
mod lch;
mod luv;
mod rgb;
mod xyz;
mod yxy;

pub use cmy::*;
pub use cmyk::*;
pub use hsl::*;
pub use hsv::*;
pub use hunter_lab::*;
pub use lab::*;
pub use lch::*;
pub use luv::*;
pub use rgb::*;
pub use xyz::*;
pub use yxy::*;

use std::f64::consts::PI;

pub trait FromRgb {
    fn from_rgb(rgb: &Rgb) -> Self;
}

pub trait ToRgb {
    fn to_rgb(&self) -> Rgb;
}

pub trait FromColor<T: ToRgb> {
    fn from_color(color: &T) -> Self;
}

pub trait Compare<T: ToRgb> {
    fn compare_euclidean(&self, color: &T) -> f64;
    fn compare_cie1976(&self, color: &T) -> f64;
    fn compare_cie2000(&self, color: &T) -> f64;
    fn compare_cmc(&self, color: &T) -> f64;
}

#[inline]
fn sqr(x: f64) -> f64 {
    x * x
}

#[inline]
fn dist(a: f64, b: f64) -> f64 {
    (a * a + b * b).sqrt()
}

#[inline]
fn deg_to_rad(deg: f64) -> f64 {
    (deg * PI) / 180.0
}

#[inline]
fn rad_to_deg(rad: f64) -> f64 {
    (rad * 180.0) / PI
}

impl<T: ToRgb, U: ToRgb + FromRgb> FromColor<T> for U {
    fn from_color(color: &T) -> Self {
        let rgb = color.to_rgb();
        U::from_rgb(&rgb)
    }
}

impl<T: ToRgb, U: ToRgb + FromRgb> Compare<T> for U {
    fn compare_euclidean(&self, color: &T) -> f64 {
        let a = self.to_rgb();
        let b = color.to_rgb();
        (
            (a.r - b.r) * (a.r - b.r) +
            (a.g - b.g) * (a.g - b.g) +
            (a.b - b.b) * (a.b - b.b)
        ).sqrt()
    }
    fn compare_cie1976(&self, color: &T) -> f64 {
        let a = Lab::from_color(self);
        let b = Lab::from_color(color);
        (
            (a.l - b.l) * (a.l - b.l) +
            (a.a - b.a) * (a.a - b.a) +
            (a.b - b.b) * (a.b - b.b)
        ).sqrt()
    }
    fn compare_cie2000(&self, color: &T) -> f64 {

        let eps: f64 = 1e-5;
        let pi2 = PI * 2.0;

        let lab_a = Lab::from_color(self);
        let lab_b = Lab::from_color(color);
        
        let c1 = dist(lab_a.a, lab_a.b);
        let c2 = dist(lab_b.a, lab_b.b);
        let mean_c = (c1 + c2) / 2.0;
        let mean_c7 = mean_c.powf(7.0);
        
        let g = 0.5 * (1.0 - (mean_c7 / (mean_c7 + 6103515625.0)).sqrt());
        let a1p = lab_a.a * (1.0 + g);
        let a2p = lab_b.a * (1.0 + g);

        let c1 = dist(a1p, lab_a.b);
        let c2 = dist(a2p, lab_b.b);
        let h1 = (lab_a.b.atan2(a1p) + pi2) % pi2;
        let h2 = (lab_b.b.atan2(a2p) + pi2) % pi2;

        let delta_l = lab_b.l - lab_a.l;
        let delta_c = c2 - c1;
        let delta_h = if c1 * c2 < eps {
            0.0
        } else if (h2 - h1).abs() <= PI {
            h2 - h1
        } else if h2 > h1 {
            h2 - h1 - pi2
        } else {
            h2 - h1 + pi2
        };
        let delta_h = 2.0 * (c1 * c2).sqrt() * (delta_h / 2.0).sin();

        let mean_l = (lab_a.l + lab_b.l) / 2.0;
        let mean_c = (c1 + c2) / 2.0;
        let mean_c7 = mean_c.powf(7.0);
        let mean_h = if c1*c2 < eps {
            h1 + h2
        } else if (h1 - h2).abs() <= PI + eps {
            (h1 + h2) / 2.0
        } else if h1 + h2 < pi2 {
            (h1 + h2 + pi2) / 2.0
        } else {
            (h1 + h2 - pi2) / 2.0
        };

        let t = 1.0
            - 0.17 * (mean_h - deg_to_rad(30.0)).cos()
            + 0.24 * (2.0 * mean_h).cos()
            + 0.32 * (3.0 * mean_h + deg_to_rad(6.0)).cos()
            - 0.2 * (4.0 * mean_h - deg_to_rad(63.0)).cos();
        
        let sl = 1.0 + (0.015 * sqr(mean_l - 50.0)) / (20.0 + sqr(mean_l - 50.0).sqrt());
        let sc = 1.0 + 0.045 * mean_c;
        let sh = 1.0 + 0.015 * mean_c * t;
        let rc = 2.0 * (mean_c7 / (mean_c7 + 6103515625.0)).sqrt();
        let rt = -(deg_to_rad(60.0 * (-sqr((rad_to_deg(mean_h) - 275.0) / 25.0)).exp())).sin() * rc;
        (
            sqr(delta_l / sl) +
            sqr(delta_c / sc) +
            sqr(delta_h / sh) +
            rt * delta_c / sc * delta_h / sh
        ).sqrt()
    }
    fn compare_cmc(&self, color: &T) -> f64 {
        let lch_a = Lch::from_color(self);
        let lch_b = Lch::from_color(color);

        let delta_l = lch_a.l - lch_b.l;
        let delta_c = lch_a.c - lch_b.c;
        let delta_h = lch_a.h - lch_b.h;

        let f = (lch_a.c.powf(4.0) / (lch_a.c.powf(4.0) + 1900.0)).sqrt();
        let t = match 164.0 <= lch_a.h && lch_a.h <= 345.0 {
            true => 0.56 + (0.2 * (lch_a.h + 168.0).cos()).abs(),
            false => 0.36 + (0.4 * (lch_a.h + 35.0).cos()).abs()
        };
        let sl = match lch_a.l < 16.0 {
            true => 0.511,
            false => 0.040975 * lch_a.l / (1.0 + 0.01765 * lch_a.l)
        };
        let sc = 0.0638 * lch_a.c / (1.0 + 0.0131 * lch_a.c) + 0.638;
        let sh = sc * (f * t + 1.0 - f);

        (
            sqr(delta_l / (2.0 * sl)) +
            sqr(delta_c / (1.0 * sc)) +
            sqr(delta_h / sh)
        ).sqrt()
    }
}

macro_rules! impl_from {
    ($from_type:ty, $($to_type:ty), *) => {
        $(
            impl From<$from_type> for $to_type {
                #[inline]
                fn from(color: $from_type) -> Self {
                    let rgb = color.to_rgb();
                    Self::from_rgb(&rgb)
                }
            }
        )*
    };
}

impl_from!(Cmy, Cmyk, Hsl, Hsv, HunterLab, Lab, Lch, Luv, Rgb, Xyz, Yxy);
impl_from!(Cmyk, Hsl, Hsv, HunterLab, Lab, Lch, Luv, Rgb, Xyz, Yxy, Cmy);
impl_from!(Hsl, Hsv, HunterLab, Lab, Lch, Luv, Rgb, Xyz, Yxy, Cmy, Cmyk);
impl_from!(Hsv, HunterLab, Lab, Lch, Luv, Rgb, Xyz, Yxy, Cmy, Cmyk, Hsl);
impl_from!(HunterLab, Lab, Lch, Luv, Rgb, Xyz, Yxy, Cmy, Cmyk, Hsl, Hsv);
impl_from!(Lab, Lch, Luv, Rgb, Xyz, Yxy, Cmy, Cmyk, Hsl, Hsv, HunterLab);
impl_from!(Lch, Luv, Rgb, Xyz, Yxy, Cmy, Cmyk, Hsl, Hsv, HunterLab, Lab);
impl_from!(Luv, Rgb, Xyz, Yxy, Cmy, Cmyk, Hsl, Hsv, HunterLab, Lab, Lch);
impl_from!(Rgb, Xyz, Yxy, Cmy, Cmyk, Hsl, Hsv, HunterLab, Lab, Lch, Luv);
impl_from!(Xyz, Yxy, Cmy, Cmyk, Hsl, Hsv, HunterLab, Lab, Lch, Luv, Rgb);
impl_from!(Yxy, Cmy, Cmyk, Hsl, Hsv, HunterLab, Lab, Lch, Luv, Rgb, Xyz);