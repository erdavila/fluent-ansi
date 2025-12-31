use fluent_ansi::{color::*, prelude::*, *};

use common::*;

mod common;

test_color_type!(
    IndexedColor(7),
    Color::Indexed(IndexedColor(7)),
    Style::new().fg(IndexedColor(7))
);

test_to_style_set_with_fg_assumed!(IndexedColor(7));

#[test]
fn indexed() {
    let color_1 = IndexedColor(7);
    assert_eq!(color_1.get_index(), 7u8);

    let color_2 = IndexedColor::new(7);
    assert_eq!(color_2.get_index(), 7u8);

    assert_eq!(color_1, color_2);
}

#[test]
fn to_style() {
    assert_eq!(
        IndexedColor(42).to_style(),
        Style::new().fg(IndexedColor(42))
    );
}
