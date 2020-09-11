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
mod approx;
mod compare;

pub use cmy::Cmy;
pub use cmyk::Cmyk;
pub use hsl::Hsl;
pub use hsv::Hsv;
pub use hunter_lab::HunterLab;
pub use lab::Lab;
pub use lch::Lch;
pub use luv::Luv;
pub use rgb::Rgb;
pub use xyz::Xyz;
pub use yxy::Yxy;
pub use approx::approx;
pub use compare::{ CompareEuclidean, CompareCie1976, CompareCie2000, CompareCmc };

pub trait FromRgb {
    /// Convert from an `Rgb` color.
    fn from_rgb(rgb: &Rgb) -> Self;
}

pub trait ToRgb {
    /// Convert into an `Rgb` color.
    fn to_rgb(&self) -> Rgb;
}

pub trait FromColor<T: ToRgb> {
    /// Convert from another color space `T`.
    fn from_color(color: &T) -> Self;
}

macro_rules! impl_from {
    ($from_type:ty, $($to_type:ty), *) => {
        impl FromColor<$from_type> for $from_type {
            #[inline]
            fn from_color(color: &Self) -> Self {
                *color
            }
        }
        $(
            impl FromColor<$from_type> for $to_type {
                #[inline]
                fn from_color(color: &$from_type) -> Self {
                    let rgb = color.to_rgb();
                    Self::from_rgb(&rgb)
                }
            }
            impl From<$from_type> for $to_type {
                #[inline]
                fn from(color: $from_type) -> Self {
                    Self::from_color(&color)
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