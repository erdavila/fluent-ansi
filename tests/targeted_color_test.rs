use fluent_ansi::{color::*, prelude::*, *};

use crate::common::*;

mod common;

test_to_style_set![
    red_fg { TargetedColor::new_for_fg(BasicColor::Red), Style::new().fg(BasicColor::Red) },
    green_fg { TargetedColor::new_for_fg(BasicColor::Green), Style::new().fg(BasicColor::Green) },
    red_bg { TargetedColor::new_for_fg(BasicColor::Red), Style::new().fg(BasicColor::Red) },
    green_bg { TargetedColor::new_for_fg(BasicColor::Green), Style::new().fg(BasicColor::Green) },
    red_underline { TargetedColor::new_for_underline(BasicColor::Red), Style::new().underline_color(BasicColor::Red) },
    green_underline { TargetedColor::new_for_underline(BasicColor::Green), Style::new().underline_color(BasicColor::Green) },
];

test_applied_to!(BasicColor::Red.for_fg(), Style::new().fg(BasicColor::Red));

#[test]
fn targeted_color() {
    let cp = TargetedColor::new(BasicColor::Red, ColorTarget::Foreground);

    assert_eq!(cp.get_color(), BasicColor::Red.to_color());
    assert_eq!(cp.get_target(), ColorTarget::Foreground);
    assert_eq!(
        cp.to_style_set(),
        Style::new().set_color(ColorTarget::Foreground, Some(BasicColor::Red))
    );
    assert_eq!(
        cp.to_style(),
        Style::new().set_color(ColorTarget::Foreground, Some(BasicColor::Red))
    );
}

#[test]
fn to_style() {
    assert_from_to!(
        to_style, Style;
        BasicColor::Red.for_fg(),
        Style::new().fg(BasicColor::Red)
    );
    assert_from_to!(
        to_style, Style;
        BasicColor::Green.for_bg(),
        Style::new().bg(BasicColor::Green)
    );
}

#[test]
fn basic_color_display() {
    assert_display!(BasicColor::Black.for_fg(), "\x1b[30m");
    assert_display!(BasicColor::Red.for_fg(), "\x1b[31m");
    assert_display!(BasicColor::Green.for_fg(), "\x1b[32m");
    assert_display!(BasicColor::Yellow.for_fg(), "\x1b[33m");
    assert_display!(BasicColor::Blue.for_fg(), "\x1b[34m");
    assert_display!(BasicColor::Magenta.for_fg(), "\x1b[35m");
    assert_display!(BasicColor::Cyan.for_fg(), "\x1b[36m");
    assert_display!(BasicColor::White.for_fg(), "\x1b[37m");

    assert_display!(BasicColor::Black.for_bg(), "\x1b[40m");
    assert_display!(BasicColor::Red.for_bg(), "\x1b[41m");
    assert_display!(BasicColor::Green.for_bg(), "\x1b[42m");
    assert_display!(BasicColor::Yellow.for_bg(), "\x1b[43m");
    assert_display!(BasicColor::Blue.for_bg(), "\x1b[44m");
    assert_display!(BasicColor::Magenta.for_bg(), "\x1b[45m");
    assert_display!(BasicColor::Cyan.for_bg(), "\x1b[46m");
    assert_display!(BasicColor::White.for_bg(), "\x1b[47m");

    assert_display!(BasicColor::Black.for_underline(), "\x1b[58;5;0m");
    assert_display!(BasicColor::Red.for_underline(), "\x1b[58;5;1m");
    assert_display!(BasicColor::Green.for_underline(), "\x1b[58;5;2m");
    assert_display!(BasicColor::Yellow.for_underline(), "\x1b[58;5;3m");
    assert_display!(BasicColor::Blue.for_underline(), "\x1b[58;5;4m");
    assert_display!(BasicColor::Magenta.for_underline(), "\x1b[58;5;5m");
    assert_display!(BasicColor::Cyan.for_underline(), "\x1b[58;5;6m");
    assert_display!(BasicColor::White.for_underline(), "\x1b[58;5;7m");
}

#[test]
fn simple_color_display() {
    assert_display!(SimpleColor::new(BasicColor::Black).for_fg(), "\x1b[30m");
    assert_display!(SimpleColor::new(BasicColor::Red).for_fg(), "\x1b[31m");
    assert_display!(SimpleColor::new(BasicColor::White).for_fg(), "\x1b[37m");

    assert_display!(SimpleColor::new(BasicColor::Black).for_bg(), "\x1b[40m");
    assert_display!(SimpleColor::new(BasicColor::Red).for_bg(), "\x1b[41m");
    assert_display!(SimpleColor::new(BasicColor::White).for_bg(), "\x1b[47m");

    assert_display!(
        SimpleColor::new(BasicColor::Black).for_underline(),
        "\x1b[58;5;0m"
    );
    assert_display!(
        SimpleColor::new(BasicColor::Red).for_underline(),
        "\x1b[58;5;1m"
    );
    assert_display!(
        SimpleColor::new(BasicColor::White).for_underline(),
        "\x1b[58;5;7m"
    );

    assert_display!(
        SimpleColor::new_bright(BasicColor::Black).for_fg(),
        "\x1b[90m"
    );
    assert_display!(
        SimpleColor::new_bright(BasicColor::Red).for_fg(),
        "\x1b[91m"
    );
    assert_display!(
        SimpleColor::new_bright(BasicColor::White).for_fg(),
        "\x1b[97m"
    );

    assert_display!(
        SimpleColor::new_bright(BasicColor::Black).for_bg(),
        "\x1b[100m"
    );
    assert_display!(
        SimpleColor::new_bright(BasicColor::Red).for_bg(),
        "\x1b[101m"
    );
    assert_display!(
        SimpleColor::new_bright(BasicColor::White).for_bg(),
        "\x1b[107m"
    );

    assert_display!(
        SimpleColor::new_bright(BasicColor::Black).for_underline(),
        "\x1b[58;5;8m"
    );
    assert_display!(
        SimpleColor::new_bright(BasicColor::Red).for_underline(),
        "\x1b[58;5;9m"
    );
    assert_display!(
        SimpleColor::new_bright(BasicColor::White).for_underline(),
        "\x1b[58;5;15m"
    );
}

#[test]
fn indexed_color_display() {
    assert_display!(IndexedColor(0).for_fg(), "\x1b[38;5;0m");
    assert_display!(IndexedColor(7).for_fg(), "\x1b[38;5;7m");
    assert_display!(IndexedColor(255).for_fg(), "\x1b[38;5;255m");

    assert_display!(IndexedColor(0).for_bg(), "\x1b[48;5;0m");
    assert_display!(IndexedColor(7).for_bg(), "\x1b[48;5;7m");
    assert_display!(IndexedColor(255).for_bg(), "\x1b[48;5;255m");

    assert_display!(IndexedColor(0).for_underline(), "\x1b[58;5;0m");
    assert_display!(IndexedColor(7).for_underline(), "\x1b[58;5;7m");
    assert_display!(IndexedColor(255).for_underline(), "\x1b[58;5;255m");
}

#[test]
fn rgb_color_display() {
    assert_display!(RGBColor::new(0, 128, 255).for_fg(), "\x1b[38;2;0;128;255m");
    assert_display!(RGBColor::new(128, 255, 0).for_fg(), "\x1b[38;2;128;255;0m");
    assert_display!(RGBColor::new(255, 0, 128).for_fg(), "\x1b[38;2;255;0;128m");

    assert_display!(RGBColor::new(0, 128, 255).for_bg(), "\x1b[48;2;0;128;255m");
    assert_display!(RGBColor::new(128, 255, 0).for_bg(), "\x1b[48;2;128;255;0m");
    assert_display!(RGBColor::new(255, 0, 128).for_bg(), "\x1b[48;2;255;0;128m");

    assert_display!(
        RGBColor::new(0, 128, 255).for_underline(),
        "\x1b[58;2;0;128;255m"
    );
    assert_display!(
        RGBColor::new(128, 255, 0).for_underline(),
        "\x1b[58;2;128;255;0m"
    );
    assert_display!(
        RGBColor::new(255, 0, 128).for_underline(),
        "\x1b[58;2;255;0;128m"
    );
}
