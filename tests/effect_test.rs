use fluent_ansi::prelude::*;

use crate::common::*;

mod common;

test_fluent_type![
    bold { Effect::Bold, Style::new().bold() },
    italic { Effect::Italic, Style::new().italic() },
];

#[test]
fn display() {
    assert_display!(Effect::Bold, "\x1b[1m");
    assert_display!(Effect::Faint, "\x1b[2m");
    assert_display!(Effect::Italic, "\x1b[3m");
    assert_display!(Effect::Underline, "\x1b[4m");
    assert_display!(Effect::CurlyUnderline, "\x1b[4:3m");
    assert_display!(Effect::DottedUnderline, "\x1b[4:4m");
    assert_display!(Effect::DashedUnderline, "\x1b[4:5m");
    assert_display!(Effect::Blink, "\x1b[5m");
    assert_display!(Effect::Reverse, "\x1b[7m");
    assert_display!(Effect::Conceal, "\x1b[8m");
    assert_display!(Effect::Strikethrough, "\x1b[9m");
    assert_display!(Effect::DoubleUnderline, "\x1b[21m");
    assert_display!(Effect::Overline, "\x1b[53m");
}
