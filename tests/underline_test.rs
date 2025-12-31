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
    assert_eq!(UnderlineStyle::Solid.to_effect(), Effect::Underline);
    assert_eq!(UnderlineStyle::Curly.to_effect(), Effect::CurlyUnderline);
    assert_eq!(UnderlineStyle::Dotted.to_effect(), Effect::DottedUnderline);
    assert_eq!(UnderlineStyle::Dashed.to_effect(), Effect::DashedUnderline);
    assert_eq!(UnderlineStyle::Double.to_effect(), Effect::DoubleUnderline);
}

#[test]
fn to_style() {
    assert_eq!(UnderlineStyle::Solid.to_style(), Style::new().underline());
    assert_eq!(
        UnderlineStyle::Curly.to_style(),
        Style::new().curly_underline()
    );
    assert_eq!(
        UnderlineStyle::Dotted.to_style(),
        Style::new().dotted_underline()
    );
    assert_eq!(
        UnderlineStyle::Dashed.to_style(),
        Style::new().dashed_underline()
    );
    assert_eq!(
        UnderlineStyle::Double.to_style(),
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
