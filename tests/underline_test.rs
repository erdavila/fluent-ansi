use fluent_ansi::prelude::*;

use common::*;

mod common;

test_fluent_type![
    solid { UnderlineEffect::Solid, Style::new().solid_underline() },
    curly { UnderlineEffect::Curly, Style::new().curly_underline() },
];

#[test]
fn to_effect() {
    assert_from_to!(
        to_effect, Effect;
        UnderlineEffect::Solid,
        Effect::SolidUnderline
    );
    assert_from_to!(
        to_effect, Effect;
        UnderlineEffect::Curly,
        Effect::CurlyUnderline
    );
    assert_from_to!(
        to_effect, Effect;
        UnderlineEffect::Dotted,
        Effect::DottedUnderline
    );
    assert_from_to!(
        to_effect, Effect;
        UnderlineEffect::Dashed,
        Effect::DashedUnderline
    );
    assert_from_to!(
        to_effect, Effect;
        UnderlineEffect::Double,
        Effect::DoubleUnderline
    );
}

#[test]
fn display() {
    assert_display!(UnderlineEffect::Solid, "\x1b[4m");
    assert_display!(UnderlineEffect::Curly, "\x1b[4:3m");
    assert_display!(UnderlineEffect::Dotted, "\x1b[4:4m");
    assert_display!(UnderlineEffect::Dashed, "\x1b[4:5m");
    assert_display!(UnderlineEffect::Double, "\x1b[21m");
}
