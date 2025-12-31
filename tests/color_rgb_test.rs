use fluent_ansi::{color::*, prelude::*, *};

use common::*;

mod common;

test_color_type!(
    RGBColor::new(0, 128, 255),
    Color::RGB(RGBColor::new(0, 128, 255)),
    Style::new().fg(RGBColor::new(0, 128, 255))
);

test_to_style_set_with_fg_assumed!(RGBColor::new(0, 128, 255));

#[test]
fn rgb() {
    let color_1 = RGBColor {
        r: 0,
        g: 128,
        b: 255,
    };
    assert_eq!(color_1.r, 0u8);
    assert_eq!(color_1.g, 128u8);
    assert_eq!(color_1.b, 255u8);

    let color_2 = RGBColor::new(0, 128, 255);
    assert_eq!(color_2.r, 0u8);
    assert_eq!(color_2.g, 128u8);
    assert_eq!(color_2.b, 255u8);

    assert_eq!(color_1, color_2);
}

#[test]
fn to_style() {
    assert_eq!(
        RGBColor::new(0, 128, 255).to_style(),
        Style::new().fg(RGBColor::new(0, 128, 255))
    );
}
