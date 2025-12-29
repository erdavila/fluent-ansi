use core::fmt::{Display, Formatter, Result};

use crate::{
    AppliedTo, Style, StyleAttribute, StyleElement, StyleSet, ToStyle, ToStyleSet, color::Color,
};

/// A color in a specific color target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TargetedColor {
    color: Color,
    target: ColorTarget,
}

impl TargetedColor {
    /// Creates a new color for a specific color target.
    #[must_use]
    pub fn new(color: impl Into<Color>, target: ColorTarget) -> Self {
        let color = color.into();
        Self { color, target }
    }

    /// Creates a new color for the foreground plane.
    #[must_use]
    pub fn new_for_fg(color: impl Into<Color>) -> Self {
        Self::new(color, ColorTarget::Foreground)
    }

    /// Creates a new color for the background plane.
    #[must_use]
    pub fn new_for_bg(color: impl Into<Color>) -> Self {
        Self::new(color, ColorTarget::Background)
    }

    /// Creates a new color for the underline effects.
    #[must_use]
    pub fn new_for_underline(color: impl Into<Color>) -> Self {
        Self::new(color, ColorTarget::Underline)
    }

    /// Gets the color.
    #[must_use]
    pub const fn get_color(self) -> Color {
        self.color
    }

    /// Gets the color target.
    #[must_use]
    pub const fn get_target(self) -> ColorTarget {
        self.target
    }
}

impl StyleElement for TargetedColor {
    fn add_to<S: StyleSet>(self, style_set: S) -> S {
        style_set.set_color(self.get_target(), Some(self.get_color()))
    }
}

impl ToStyleSet for TargetedColor {
    type StyleSet = Style;

    fn to_style_set(self) -> Self::StyleSet {
        self.to_style()
    }
}

impl ToStyle for TargetedColor {
    fn to_style(self) -> Style {
        self.into()
    }
}

impl AppliedTo for TargetedColor {}

impl Display for TargetedColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.to_style().fmt(f)
    }
}

impl<C: Into<Color>> From<C> for TargetedColor {
    fn from(value: C) -> Self {
        Self::new(value, ColorTarget::Foreground)
    }
}

/// The target where a color is applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorTarget {
    /// The foreground plane.
    Foreground,
    /// The background plane.
    Background,
    /// The underline effects.
    Underline,
}

impl StyleAttribute for ColorTarget {
    type Value = Option<Color>;

    fn set_in<S: StyleSet>(self, style_set: S, value: Self::Value) -> S {
        style_set.set_color(self, value)
    }

    fn get_from<S: StyleSet>(self, style_set: &S) -> Self::Value {
        style_set.get_color(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        StyleSet as _,
        color::{BasicColor, ColorKind as _, IndexedColor, RGBColor, SimpleColor},
        tests::assert_display,
        to_style_set::tests::test_to_style_set_methods,
    };

    use super::*;

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

    test_to_style_set_methods!(red_fg; TargetedColor::new_for_fg(BasicColor::Red), Style::new().fg(BasicColor::Red));
    test_to_style_set_methods!(green_fg; TargetedColor::new_for_fg(BasicColor::Green), Style::new().fg(BasicColor::Green));
    test_to_style_set_methods!(red_bg; TargetedColor::new_for_fg(BasicColor::Red), Style::new().fg(BasicColor::Red));
    test_to_style_set_methods!(green_bg; TargetedColor::new_for_fg(BasicColor::Green), Style::new().fg(BasicColor::Green));
    test_to_style_set_methods!(red_underline; TargetedColor::new_for_underline(BasicColor::Red), Style::new().underline_color(BasicColor::Red));
    test_to_style_set_methods!(green_underline; TargetedColor::new_for_underline(BasicColor::Green), Style::new().underline_color(BasicColor::Green));

    #[test]
    fn applied_to() {
        let stld = BasicColor::Red.for_fg().applied_to("CONTENT");

        assert_eq!(stld.get_content(), &"CONTENT");
        assert_eq!(stld.get_style(), Style::new().fg(BasicColor::Red));
    }

    #[test]
    fn to_style() {
        assert_eq!(
            BasicColor::Red.for_fg().to_style(),
            Style::new().fg(BasicColor::Red)
        );
        assert_eq!(
            BasicColor::Green.for_bg().to_style(),
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
}
