use fluent_ansi::{AppliedTo as _, Style, ToStyle as _, ToStyleSet as _, color::*};

use common::*;

mod common;

test_color_methods!(
    BasicColor::Red,
    Color::Simple(SimpleColor::new(BasicColor::Red)),
    Style::new().fg(BasicColor::Red)
);

test_to_style_set_with_fg_assumed!(BasicColor::Red);

#[test]
fn bright() {
    assert_eq!(
        BasicColor::Red.bright(),
        SimpleColor::new_bright(BasicColor::Red)
    );
}

#[test]
fn applied_to() {
    let stld = BasicColor::Red.applied_to("CONTENT");

    assert_eq!(stld.get_content(), &"CONTENT");
    assert_eq!(stld.get_style(), Style::new().fg(BasicColor::Red));
}

#[test]
fn to_simple_color() {
    assert_eq!(
        BasicColor::Red.to_simple_color(),
        SimpleColor::new(BasicColor::Red)
    );
}

#[test]
fn to_style() {
    assert_eq!(BasicColor::Red.to_style(), Style::new().fg(BasicColor::Red));
}
