use fluent_ansi::{prelude::*, *};

use common::*;

mod common;

#[test]
fn reset() {
    assert_display!(Reset, "\x1b[0m");
}

#[test]
fn eq() {
    assert_eq!(Reset, Reset);
    assert_eq!(Reset, Style::new());
    assert_ne!(Reset, Style::new().bold());
    assert_eq!(Style::new(), Reset);
    assert_ne!(Style::new().bold(), Reset);
}

#[test]
fn to_style() {
    assert_from_to!(
        to_style, Style;
        Reset,
        Style::new()
    );
}
