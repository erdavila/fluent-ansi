use fluent_ansi::color::*;

use common::*;

mod common;

test_color_type![
    simple {
        Color::Simple(SimpleColor::new(BasicColor::Red)),
        Color::Simple(SimpleColor::new(BasicColor::Red)),
        Style::new().fg(BasicColor::Red)
    },
    indexed {
        Color::Indexed(IndexedColor(42)),
        Color::Indexed(IndexedColor(42)),
        Style::new().fg(IndexedColor(42))
    },
    rgb {
        Color::RGB(RGBColor::new(0, 128, 255)),
        Color::RGB(RGBColor::new(0, 128, 255)),
        Style::new().fg(RGBColor::new(0, 128, 255))
    },
];

test_to_style_set_with_fg_assumed![
    simple { Color::Simple(SimpleColor::new(BasicColor::Red)) },
    indexed { Color::Indexed(IndexedColor(42)) },
    rgb { Color::RGB(RGBColor::new(0, 128, 255)) },
];

#[test]
fn basic() {
    macro_rules! assert_basic_color_shortcut {
        ($shortcut:expr, $expected:expr) => {{
            // The returned type must be BasicColor instead of Color
            let color: BasicColor = $shortcut;
            assert_eq!(color, $expected);
        }};
    }

    assert_basic_color_shortcut!(Color::BLACK, BasicColor::Black);
    assert_basic_color_shortcut!(Color::RED, BasicColor::Red);
    assert_basic_color_shortcut!(Color::GREEN, BasicColor::Green);
    assert_basic_color_shortcut!(Color::YELLOW, BasicColor::Yellow);
    assert_basic_color_shortcut!(Color::BLUE, BasicColor::Blue);
    assert_basic_color_shortcut!(Color::MAGENTA, BasicColor::Magenta);
    assert_basic_color_shortcut!(Color::CYAN, BasicColor::Cyan);
    assert_basic_color_shortcut!(Color::WHITE, BasicColor::White);
}

#[test]
fn indexed() {
    // The returned type must be RGBColor instead of Color
    let color: IndexedColor = Color::indexed(127);
    assert_eq!(color, IndexedColor(127));
}

#[test]
fn rgb() {
    // The returned type must be RGBColor instead of Color
    let color: RGBColor = Color::rgb(0, 128, 255);
    assert_eq!(color, RGBColor::new(0, 128, 255));
}

#[test]
fn from_basic_color() {
    assert_eq!(
        Color::from(BasicColor::Red),
        Color::Simple(SimpleColor::new(BasicColor::Red))
    );
}

#[test]
fn from_simple_color() {
    assert_eq!(
        Color::from(SimpleColor::new(BasicColor::Red)),
        Color::Simple(SimpleColor::new(BasicColor::Red))
    );
}

#[test]
fn from_indexed_color() {
    assert_eq!(
        Color::from(IndexedColor(7)),
        Color::Indexed(IndexedColor(7))
    );
}

#[test]
fn from_rgb() {
    assert_eq!(
        Color::from(RGBColor::new(0, 128, 255)),
        Color::RGB(RGBColor::new(0, 128, 255))
    );
}
