use core::fmt::{Display, Formatter, Result};

use crate::{
    AppliedTo, Style, StyleAttribute, StyleElement, StyleSet as _, ToStyle, ToStyleSet,
    color::Color,
};

/// A color in a specific plane (foreground or background).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColorInAPlane {
    color: Color,
    plane: Plane,
}

impl ColorInAPlane {
    /// Creates a new color in a specific plane.
    #[must_use]
    pub fn new(color: impl Into<Color>, plane: Plane) -> Self {
        Self {
            color: color.into(),
            plane,
        }
    }

    /// Creates a new color in the foreground plane.
    #[must_use]
    pub fn new_in_fg(color: impl Into<Color>) -> Self {
        Self::new(color, Plane::Foreground)
    }

    /// Creates a new color in the background plane.
    #[must_use]
    pub fn new_in_bg(color: impl Into<Color>) -> Self {
        Self::new(color, Plane::Background)
    }

    /// Gets the color.
    #[must_use]
    pub const fn get_color(self) -> Color {
        self.color
    }

    /// Gets the plane.
    #[must_use]
    pub const fn get_plane(self) -> Plane {
        self.plane
    }
}

impl StyleElement for ColorInAPlane {
    fn add_to_style(self, style: Style) -> Style {
        style.set_color(self.plane, Some(self.color))
    }
}

impl ToStyleSet for ColorInAPlane {
    type StyleSet = Style;

    fn to_style_set(self) -> Self::StyleSet {
        self.to_style()
    }
}

impl ToStyle for ColorInAPlane {
    fn to_style(self) -> Style {
        self.into()
    }
}

impl AppliedTo for ColorInAPlane {}

impl Display for ColorInAPlane {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.to_style().fmt(f)
    }
}

/// The plane where a color is applied: foreground or background.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Plane {
    /// The foreground plane.
    Foreground,
    /// The background plane.
    Background,
}

impl StyleAttribute for Plane {
    type Value = Option<Color>;

    fn set_in_style(self, style: Style, value: Self::Value) -> Style {
        match self {
            Plane::Foreground => Style { fg: value, ..style },
            Plane::Background => Style { bg: value, ..style },
        }
    }

    fn get_from_style(self, style: &Style) -> Self::Value {
        match self {
            Plane::Foreground => style.fg,
            Plane::Background => style.bg,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Effect, StyleSet as _, assert_display,
        color::{BasicColor, ColorKind as _, IndexedColor, RGBColor, SimpleColor},
    };

    use super::*;

    #[test]
    fn color_in_a_plane() {
        let cp = ColorInAPlane::new(BasicColor::Red, Plane::Foreground);

        assert_eq!(cp.get_color(), BasicColor::Red.to_color());
        assert_eq!(cp.get_plane(), Plane::Foreground);
        assert_eq!(
            cp.to_style_set(),
            Style::new().set_color(Plane::Foreground, Some(BasicColor::Red))
        );
        assert_eq!(
            cp.to_style(),
            Style::new().set_color(Plane::Foreground, Some(BasicColor::Red))
        );
    }

    #[test]
    fn effect() {
        let color_in_a_plane = BasicColor::Red.in_fg();

        assert_eq!(
            color_in_a_plane.bold(),
            Style::new().fg(BasicColor::Red).bold()
        );
        assert_eq!(
            color_in_a_plane.effect(Effect::Bold),
            Style::new().fg(BasicColor::Red).bold()
        );
        assert_eq!(
            color_in_a_plane.add(Effect::Bold),
            Style::new().fg(BasicColor::Red).bold()
        );
    }
    #[test]
    fn add_color() {
        let color_in_a_plane = BasicColor::Red.in_fg();

        assert_eq!(
            color_in_a_plane.fg(BasicColor::Green),
            Style::new().fg(BasicColor::Green)
        );
        assert_eq!(
            color_in_a_plane.bg(BasicColor::Green),
            Style::new().fg(BasicColor::Red).bg(BasicColor::Green)
        );
        assert_eq!(
            color_in_a_plane.color(BasicColor::Green.in_bg()),
            Style::new().fg(BasicColor::Red).bg(BasicColor::Green)
        );
        assert_eq!(
            color_in_a_plane.add(BasicColor::Green.in_bg()),
            Style::new().fg(BasicColor::Red).bg(BasicColor::Green)
        );
    }

    #[test]
    fn applied_to() {
        let stld = BasicColor::Red.in_fg().applied_to("CONTENT");

        assert_eq!(stld.get_content(), &"CONTENT");
        assert_eq!(stld.get_style(), Style::new().fg(BasicColor::Red));
    }

    #[test]
    fn to_style() {
        assert_eq!(
            BasicColor::Red.in_fg().to_style(),
            Style::new().fg(BasicColor::Red)
        );
        assert_eq!(
            BasicColor::Green.in_bg().to_style(),
            Style::new().bg(BasicColor::Green)
        );
    }

    #[test]
    fn display() {
        assert_display!(BasicColor::Black.in_fg(), "\x1b[30m");
        assert_display!(BasicColor::Red.in_fg(), "\x1b[31m");
        assert_display!(BasicColor::Green.in_fg(), "\x1b[32m");
        assert_display!(BasicColor::Yellow.in_fg(), "\x1b[33m");
        assert_display!(BasicColor::Blue.in_fg(), "\x1b[34m");
        assert_display!(BasicColor::Magenta.in_fg(), "\x1b[35m");
        assert_display!(BasicColor::Cyan.in_fg(), "\x1b[36m");
        assert_display!(BasicColor::White.in_fg(), "\x1b[37m");

        assert_display!(BasicColor::Black.in_bg(), "\x1b[40m");
        assert_display!(BasicColor::Red.in_bg(), "\x1b[41m");
        assert_display!(BasicColor::Green.in_bg(), "\x1b[42m");
        assert_display!(BasicColor::Yellow.in_bg(), "\x1b[43m");
        assert_display!(BasicColor::Blue.in_bg(), "\x1b[44m");
        assert_display!(BasicColor::Magenta.in_bg(), "\x1b[45m");
        assert_display!(BasicColor::Cyan.in_bg(), "\x1b[46m");
        assert_display!(BasicColor::White.in_bg(), "\x1b[47m");

        assert_display!(SimpleColor::new(BasicColor::Black).in_fg(), "\x1b[30m");
        assert_display!(SimpleColor::new(BasicColor::Red).in_fg(), "\x1b[31m");
        assert_display!(SimpleColor::new(BasicColor::White).in_fg(), "\x1b[37m");

        assert_display!(SimpleColor::new(BasicColor::Black).in_bg(), "\x1b[40m");
        assert_display!(SimpleColor::new(BasicColor::Red).in_bg(), "\x1b[41m");
        assert_display!(SimpleColor::new(BasicColor::White).in_bg(), "\x1b[47m");

        assert_display!(
            SimpleColor::new_bright(BasicColor::Black).in_fg(),
            "\x1b[90m"
        );
        assert_display!(SimpleColor::new_bright(BasicColor::Red).in_fg(), "\x1b[91m");
        assert_display!(
            SimpleColor::new_bright(BasicColor::White).in_fg(),
            "\x1b[97m"
        );

        assert_display!(
            SimpleColor::new_bright(BasicColor::Black).in_bg(),
            "\x1b[100m"
        );
        assert_display!(
            SimpleColor::new_bright(BasicColor::Red).in_bg(),
            "\x1b[101m"
        );
        assert_display!(
            SimpleColor::new_bright(BasicColor::White).in_bg(),
            "\x1b[107m"
        );

        assert_display!(IndexedColor(0).in_fg(), "\x1b[38;5;0m");
        assert_display!(IndexedColor(7).in_fg(), "\x1b[38;5;7m");
        assert_display!(IndexedColor(255).in_fg(), "\x1b[38;5;255m");

        assert_display!(IndexedColor(0).in_bg(), "\x1b[48;5;0m");
        assert_display!(IndexedColor(7).in_bg(), "\x1b[48;5;7m");
        assert_display!(IndexedColor(255).in_bg(), "\x1b[48;5;255m");

        assert_display!(RGBColor::new(0, 128, 255).in_fg(), "\x1b[38;2;0;128;255m");
        assert_display!(RGBColor::new(128, 255, 0).in_fg(), "\x1b[38;2;128;255;0m");
        assert_display!(RGBColor::new(255, 0, 128).in_fg(), "\x1b[38;2;255;0;128m");

        assert_display!(RGBColor::new(0, 128, 255).in_bg(), "\x1b[48;2;0;128;255m");
        assert_display!(RGBColor::new(128, 255, 0).in_bg(), "\x1b[48;2;128;255;0m");
        assert_display!(RGBColor::new(255, 0, 128).in_bg(), "\x1b[48;2;255;0;128m");
    }
}
