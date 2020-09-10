# color_space
A Rust library for converting between color spaces and comparing colors, ported from https://github.com/berendeanicolae/ColorSpace.

**WARNING:** This has not been extensively tested yet, it's simply been ported, but there could still be some errors in my ported code for certain conversions. Until version `1.0.0`, which will be stable, use at your own discretion.

## Color Conversion
You can convert between any supported color spaces using the `from` trait method:
```rust
let rgb = Rgb::new(0.0, 255.0, 0.0);
let hsv = Hsv::from(rgb);
assert_eq!(hsv, Hsv::new(120.0, 1.0, 1.0));
```

If you prefer to do it by reference, you can use the `from_color` method:
```rust
let rgb = Rgb::new(0.0, 0.0, 255.0);
let hsv = Hsv::from_color(&rgb);
assert_eq!(hsv, Hsv::new(240.0, 1.0, 1.0));
```

## Comparing Colors
You can compare colors by using the `compare_*` methods:
```rust
let rgb = Rgb::new(255.0, 0.0, 0.0);
let hsv = Hsv::new(0.0, 1.0, 1.0);
let diff = rgb.compare_cie2000(&hsv);
assert_eq!(diff, 0.0);
// these two colors are the same, so the difference is zero
```

## Currently Supported Color Spaces
* CMY
* CMYK
* HSL
* HSB
* HSV
* CIE L*AB
* Hunter LAB
* LCH
* LUV
* RGB
* XYZ
* YXY

## Currently Supported Comparisons
* Euclidean
* CIE1976
* CIE2000
* CMC