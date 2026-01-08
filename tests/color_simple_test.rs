use fluent_ansi::color::*;

use common::*;

mod common;

test_color_type![
    non_bright {
        SimpleColor::new(Color::RED),
        Color::Simple(SimpleColor::new(BasicColor::Red)),
        Style::new().fg(SimpleColor::new(BasicColor::Red))
    },
    bright {
        SimpleColor::new_bright(Color::RED),
        Color::Simple(SimpleColor::new_bright(BasicColor::Red)),
        Style::new().fg(SimpleColor::new_bright(BasicColor::Red))
    },
];

#[test]
fn new() {
    let color = SimpleColor::new(BasicColor::Red);

    assert_eq!(color.get_basic_color(), BasicColor::Red);
    assert_eq!(color.is_bright(), false);
}

#[test]
fn new_bright() {
    let color = SimpleColor::new_bright(BasicColor::Red);

    assert_eq!(color.get_basic_color(), BasicColor::Red);
    assert_eq!(color.is_bright(), true);
}

#[test]
fn bright() {
    let simple_regular_color = SimpleColor::new(BasicColor::Red);
    let simple_bright_color = SimpleColor::new_bright(BasicColor::Red);

    assert_eq!(simple_regular_color.bright(), simple_bright_color);
    assert_eq!(simple_bright_color.bright(), simple_bright_color);
}
