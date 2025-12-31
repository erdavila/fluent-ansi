use fluent_ansi::color::*;

use common::*;

mod common;

test_color_type!(
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
fn to_simple_color() {
    assert_eq!(
        BasicColor::Red.to_simple_color(),
        SimpleColor::new(BasicColor::Red)
    );
}
