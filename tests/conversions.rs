use color_space::*;
use std::fmt::*;

fn test_conversion<FromType, ToType>(input: FromType, expected: ToType)
where
    FromType: Debug + PartialEq + ToRgb,
    ToType: Debug + PartialEq + ToRgb + FromColor<FromType>,
{
    let a = input.to_rgb();
    let b = expected.to_rgb();
    assert_eq!(a, b);
}

#[test]
fn test_convert_cmy_cmy() {
    test_conversion(
        Cmy::new(0.72549, 0.49020, 0.29412),
        Cmy::new(0.72549, 0.49020, 0.29412)
    );
}

#[test]
fn test_convert_cmy_cmyk() {
    test_conversion(
        Cmy::new(0.72549, 0.49020, 0.29412),
        Cmyk::new(0.61110953703179, 0.2777809259364198, 0.0, 0.29411999999999994)
    );
}

#[test]
fn test_convert_cmy_hsl() {
    test_conversion(
        Cmy::new(0.72549, 0.49020, 0.29412),
        Hsl::new(207.5229325111681, 43.77509842266802, 48.823530083777854)
    );
}

#[test]
fn test_convert_cmy_hsv() {
    test_conversion(
        Cmy::new(0.72549, 0.49020, 0.29412),
        Hsv::new(207.5229325111681, 0.61110953703179, 0.7058800000000001)
    );
}

#[test]
fn test_convert_cmy_hunterlab() {
    test_conversion(
        Cmy::new(0.72549, 0.49020, 0.29412),
        HunterLab::new(45.345731409137045, -5.561074840212549, -28.61341564730304)
    );
}

#[test]
fn test_convert_cmy_lab() {
    test_conversion(
        Cmy::new(0.72549, 0.49020, 0.29412),
        Lab::new(52.467, -4.070, -32.198)
    );
}

#[test]
fn test_convert_cmy_lch() {
    test_conversion(
        Cmy::new(0.72549, 0.49020, 0.29412),
        Lch::new(52.467152054850615, 32.4544987607236, 262.79619174958583)
    );
}

#[test]
fn test_convert_cmy_luv() {
    test_conversion(
        Cmy::new(0.72549, 0.49020, 0.29412),
        Luv::new(52.467152054850615, -25.10699263443856, -48.3742052452972)
    );
}

#[test]
fn test_convert_cmy_rgb() {
    test_conversion(
        Cmy::new(0.72549, 0.49020, 0.29412),
        Rgb::new(70.0, 130.0, 180.0)
    );
}

#[test]
fn test_convert_cmy_xyz() {
    test_conversion(
        Cmy::new(0.72549, 0.49020, 0.29412),
        Xyz::new(18.74644744398548, 20.56235357029598, 46.16058375040178)
    );
}

#[test]
fn test_convert_cmy_yxy() {
    test_conversion(
        Cmy::new(0.72549, 0.49020, 0.29412),
        Yxy::new(20.56235357029598, 0.2193352332604094, 0.24058150912059142)
    );
}