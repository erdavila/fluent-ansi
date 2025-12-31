use fluent_ansi::{prelude::*, *};

use common::*;

mod common;

test_to_style_set![
    solid { UnderlineStyle::Solid, Style::new().underline() },
    curly { UnderlineStyle::Curly, Style::new().curly_underline() },
];

test_applied_to!(UnderlineStyle::Curly, Style::new().curly_underline());

#[test]
fn to_effect() {
    assert_from_to!(
        to_effect, Effect;
        UnderlineStyle::Solid,
        Effect::Underline
    );
    assert_from_to!(
        to_effect, Effect;
        UnderlineStyle::Curly,
        Effect::CurlyUnderline
    );
    assert_from_to!(
        to_effect, Effect;
        UnderlineStyle::Dotted,
        Effect::DottedUnderline
    );
    assert_from_to!(
        to_effect, Effect;
        UnderlineStyle::Dashed,
        Effect::DashedUnderline
    );
    assert_from_to!(
        to_effect, Effect;
        UnderlineStyle::Double,
        Effect::DoubleUnderline
    );
}

#[test]
fn to_style() {
    assert_from_to!(
        to_style, Style;
        UnderlineStyle::Solid,
        Style::new().underline()
    );
    assert_from_to!(
        to_style, Style;
        UnderlineStyle::Curly,
        Style::new().curly_underline()
    );
    assert_from_to!(
        to_style, Style;
        UnderlineStyle::Dotted,
        Style::new().dotted_underline()
    );
    assert_from_to!(
        to_style, Style;
        UnderlineStyle::Dashed,
        Style::new().dashed_underline()
    );
    assert_from_to!(
        to_style, Style;
        UnderlineStyle::Double,
        Style::new().double_underline()
    );
}

#[test]
fn display() {
    assert_display!(UnderlineStyle::Solid, "\x1b[4m");
    assert_display!(UnderlineStyle::Curly, "\x1b[4:3m");
    assert_display!(UnderlineStyle::Dotted, "\x1b[4:4m");
    assert_display!(UnderlineStyle::Dashed, "\x1b[4:5m");
    assert_display!(UnderlineStyle::Double, "\x1b[21m");
}
