use fluent_ansi::{color::*, prelude::*, *};

use common::*;

mod common;

test_to_style_set!(Style::new(), Style::new());
test_style_set!(Style::new());
test_applied_to!(Style::new().bold(), Style::new().bold());

#[test]
fn effects_display() {
    let style = Style::new();

    assert_display!(style, "\x1b[0m");
    assert_display!(style.bold(), "\x1b[1m");
    assert_display!(style.faint(), "\x1b[2m");
    assert_display!(style.italic(), "\x1b[3m");
    assert_display!(style.underline(), "\x1b[4m");
    assert_display!(style.curly_underline(), "\x1b[4:3m");
    assert_display!(style.dotted_underline(), "\x1b[4:4m");
    assert_display!(style.dashed_underline(), "\x1b[4:5m");
    assert_display!(style.blink(), "\x1b[5m");
    assert_display!(style.reverse(), "\x1b[7m");
    assert_display!(style.conceal(), "\x1b[8m");
    assert_display!(style.strikethrough(), "\x1b[9m");
    assert_display!(style.double_underline(), "\x1b[21m");
    assert_display!(style.overline(), "\x1b[53m");
}

#[test]
fn colors_display() {
    let style = Style::new();

    assert_display!(style.fg(BasicColor::Red), "\x1b[31m");
    assert_display!(style.bg(BasicColor::Red), "\x1b[41m");
}

#[test]
fn combined_display() {
    let style = Style::new()
        .bold()
        .fg(BasicColor::Red)
        .underline()
        .bg(BasicColor::Green);
    assert_display!(style, "\x1b[1;4;31;42m");
}

#[test]
fn default() {
    assert_display!(Style::default(), "\x1b[0m");
}

#[test]
fn to_style() {
    let style = Style::new().bold().fg(BasicColor::Red);
    assert_eq!(style.to_style(), style);
}

#[test]
fn from_targeted_color() {
    assert_eq!(
        Style::from(BasicColor::Red.for_fg()),
        Style::new().color(BasicColor::Red.for_fg())
    );
}
