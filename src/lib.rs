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
pub use compare::Compare;

pub trait FromRgb {
    fn from_rgb(rgb: &Rgb) -> Self;
}

pub trait ToRgb {
    fn to_rgb(&self) -> Rgb;
}

pub trait FromColor<T: ToRgb> {
    fn from_color(color: &T) -> Self;
}

impl<T: ToRgb, U: ToRgb + FromRgb> FromColor<T> for U {
    fn from_color(color: &T) -> Self {
        let rgb = color.to_rgb();
        U::from_rgb(&rgb)
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

#[test]
fn tests() {
    let col1 = Rgb::new(255.0, 0.0, 0.0);
    let col2 = Hsv::new(0.0, 1.0, 1.0);
    let diff = col1.compare_cie2000(&col2);
    assert_eq!(diff, 0.0);
}