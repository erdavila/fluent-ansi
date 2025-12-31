use fluent_ansi::{color::*, prelude::*, *};

use common::*;

mod common;

test_to_style_set!(Styled::new("CONTENT"), Styled::new("CONTENT"));

test_style_set!(Styled::new("CONTENT"));

#[test]
fn content_and_style() {
    let styled = Styled::new("CONTENT").bold();
    assert_eq!(styled.get_content(), &"CONTENT");
    assert_eq!(styled.get_style(), Style::new().bold());

    let styled = styled.bold().with_content("NEW CONTENT");
    assert_eq!(styled.get_content(), &"NEW CONTENT");
    assert_eq!(styled.get_style(), Style::new().bold());

    let styled = styled.with_style(Style::new().fg(BasicColor::Red));
    assert_eq!(styled.get_content(), &"NEW CONTENT");
    assert_eq!(styled.get_style(), Style::new().fg(BasicColor::Red));

    let content = styled.into_content();
    assert_eq!(content, "NEW CONTENT");
}

#[test]
fn effects_display() {
    let styled = Styled::new("CONTENT");

    assert_display!(styled, "CONTENT");
    assert_display!(styled.bold(), "\x1b[1mCONTENT\x1b[0m");
    assert_display!(styled.faint(), "\x1b[2mCONTENT\x1b[0m");
    assert_display!(styled.italic(), "\x1b[3mCONTENT\x1b[0m");
    assert_display!(styled.underline(), "\x1b[4mCONTENT\x1b[0m");
    assert_display!(styled.curly_underline(), "\x1b[4:3mCONTENT\x1b[0m");
    assert_display!(styled.dotted_underline(), "\x1b[4:4mCONTENT\x1b[0m");
    assert_display!(styled.dashed_underline(), "\x1b[4:5mCONTENT\x1b[0m");
    assert_display!(styled.blink(), "\x1b[5mCONTENT\x1b[0m");
    assert_display!(styled.reverse(), "\x1b[7mCONTENT\x1b[0m");
    assert_display!(styled.conceal(), "\x1b[8mCONTENT\x1b[0m");
    assert_display!(styled.strikethrough(), "\x1b[9mCONTENT\x1b[0m");
    assert_display!(styled.double_underline(), "\x1b[21mCONTENT\x1b[0m");
    assert_display!(styled.overline(), "\x1b[53mCONTENT\x1b[0m");
}

#[test]
fn colors_display() {
    let styled = Styled::new("CONTENT");

    assert_display!(styled.fg(BasicColor::Red), "\x1b[31mCONTENT\x1b[0m");
    assert_display!(styled.bg(BasicColor::Red), "\x1b[41mCONTENT\x1b[0m");
}

#[test]
fn combined_display() {
    let styled = Styled::new("CONTENT")
        .bold()
        .fg(BasicColor::Red)
        .underline()
        .bg(BasicColor::Green);
    assert_display!(styled, "\x1b[1;4;31;42mCONTENT\x1b[0m");
}
