use fluent_ansi::color::*;

use common::*;

mod common;

test_color_type!(
    BasicColor::Red,
    Color::Simple(SimpleColor::new(BasicColor::Red)),
    Style::new().fg(BasicColor::Red)
);

#[test]
fn bright() {
    assert_eq!(
        BasicColor::Red.bright(),
        SimpleColor::new_bright(BasicColor::Red)
    );
}

#[test]
fn to_simple_color() {
    assert_from_to!(
        to_simple_color, SimpleColor;
        BasicColor::Red,
        SimpleColor::new(BasicColor::Red)
    );
}
