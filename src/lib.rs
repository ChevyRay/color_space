//! A library for converting between color spaces and comparing colors, ported from https://github.com/berendeanicolae/ColorSpace.
//! 
//! ## Color Conversion
//! You can convert between any supported color spaces using the `from` trait method:
//! ```rust
//! use color_space::{Rgb, Hsv};
//! let rgb = Rgb::new(0.0, 255.0, 0.0);
//! let hsv = Hsv::from(rgb);
//! assert_eq!(hsv, Hsv::new(120.0, 1.0, 1.0));
//! ```
//! 
//! You can also do this generically with the `from_color` method:
//! ```rust
//! use color_space::{Rgb, Hsv, FromColor};
//! let rgb = Rgb::new(0.0, 0.0, 255.0);
//! let hsv = Hsv::from_color(&rgb);
//! assert_eq!(hsv, Hsv::new(240.0, 1.0, 1.0));
//! ```
//! 
//! ## Comparing Colors
//! You can compare colors by using the `compare_*` methods:
//! ```rust
//! use color_space::{Rgb, Hsv, CompareCie2000};
//! let rgb = Rgb::new(255.0, 0.0, 0.0);
//! let hsv = Hsv::new(0.0, 1.0, 1.0);
//! let diff = rgb.compare_cie2000(&hsv);
//! assert_eq!(diff, 0.0);
//! // these two colors are the same, so the difference is zero
//! ```

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
pub(crate) use approx::approx;
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