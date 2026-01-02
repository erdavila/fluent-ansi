use fluent_ansi::color::*;

use common::*;

mod common;

test_color_type!(
    IndexedColor(7),
    Color::Indexed(IndexedColor(7)),
    Style::new().fg(IndexedColor(7))
);

#[test]
fn indexed() {
    let color_1 = IndexedColor(7);
    assert_eq!(color_1.get_index(), 7u8);

    let color_2 = IndexedColor::new(7);
    assert_eq!(color_2.get_index(), 7u8);

    assert_eq!(color_1, color_2);
}
