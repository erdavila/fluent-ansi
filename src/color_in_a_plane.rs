use core::fmt::{Display, Formatter, Result};

use crate::{
    AppliedTo, Color, Format, FormatElement, FormatSet as _, Position, ToFormat, ToFormatSet,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColorInAPlane {
    color: Color,
    plane: Plane,
}

impl ColorInAPlane {
    #[must_use]
    pub fn new(color: impl Into<Color>, plane: Plane) -> Self {
        Self {
            color: color.into(),
            plane,
        }
    }

    #[must_use]
    pub fn new_in_fg(color: impl Into<Color>) -> Self {
        Self::new(color, Plane::Foreground)
    }

    #[must_use]
    pub fn new_in_bg(color: impl Into<Color>) -> Self {
        Self::new(color, Plane::Background)
    }

    #[must_use]
    pub fn get_color(self) -> Color {
        self.color
    }

    #[must_use]
    pub fn get_plane(self) -> Plane {
        self.plane
    }
}

impl FormatElement for ColorInAPlane {
    fn add_to_format(self, format: Format) -> Format {
        format.set_color(self.plane, Some(self.color))
    }
}

impl ToFormatSet for ColorInAPlane {
    type FormatSet = Format;

    fn to_format_set(self) -> Self::FormatSet {
        self.to_format()
    }
}

impl ToFormat for ColorInAPlane {
    fn to_format(self) -> Format {
        self.into()
    }
}

impl AppliedTo for ColorInAPlane {}

impl Display for ColorInAPlane {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.to_format().fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Plane {
    Foreground,
    Background,
}

impl Position for Plane {
    type Value = Option<Color>;

    fn set_in_format(self, format: Format, value: Self::Value) -> Format {
        match self {
            Plane::Foreground => Format {
                fg: value,
                ..format
            },
            Plane::Background => Format {
                bg: value,
                ..format
            },
        }
    }

    fn get_from_format(self, format: &Format) -> Self::Value {
        match self {
            Plane::Foreground => format.fg,
            Plane::Background => format.bg,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{BasicColor, ColorKind, EightBitColor, Flag, FormatSet as _, assert_display};

    use super::*;

    #[test]
    fn color_in_a_plane() {
        let cp = ColorInAPlane::new(BasicColor::Red, Plane::Foreground);

        assert_eq!(cp.get_color(), Color::Basic(BasicColor::Red));
        assert_eq!(cp.get_plane(), Plane::Foreground);
        assert_eq!(
            cp.to_format_set(),
            Format::new().set_color(Plane::Foreground, Some(BasicColor::Red))
        );
        assert_eq!(
            cp.to_format(),
            Format::new().set_color(Plane::Foreground, Some(BasicColor::Red))
        );
    }

    #[test]
    fn flag() {
        let color_in_a_plane = BasicColor::Red.in_fg();

        assert_eq!(
            color_in_a_plane.bold(),
            Format::new().fg(BasicColor::Red).bold()
        );
        assert_eq!(
            color_in_a_plane.flag(Flag::Bold),
            Format::new().fg(BasicColor::Red).bold()
        );
        assert_eq!(
            color_in_a_plane.add(Flag::Bold),
            Format::new().fg(BasicColor::Red).bold()
        );
    }
    #[test]
    fn color_in_a_plane_add_color() {
        let color_in_a_plane = BasicColor::Red.in_fg();

        assert_eq!(
            color_in_a_plane.fg(BasicColor::Green),
            Format::new().fg(BasicColor::Green)
        );
        assert_eq!(
            color_in_a_plane.bg(BasicColor::Green),
            Format::new().fg(BasicColor::Red).bg(BasicColor::Green)
        );
        assert_eq!(
            color_in_a_plane.color(BasicColor::Green.in_bg()),
            Format::new().fg(BasicColor::Red).bg(BasicColor::Green)
        );
        assert_eq!(
            color_in_a_plane.add(BasicColor::Green.in_bg()),
            Format::new().fg(BasicColor::Red).bg(BasicColor::Green)
        );
    }

    #[test]
    fn color_in_a_plane_applied_to() {
        let fmtd = BasicColor::Red.in_fg().applied_to("CONTENT");

        assert_eq!(fmtd.get_content(), &"CONTENT");
        assert_eq!(fmtd.get_format(), Format::new().fg(BasicColor::Red));
    }

    #[test]
    fn color_in_a_plane_to_format() {
        assert_eq!(
            BasicColor::Red.in_fg().to_format(),
            Format::new().fg(BasicColor::Red)
        );
        assert_eq!(
            BasicColor::Green.in_bg().to_format(),
            Format::new().bg(BasicColor::Green)
        );
    }

    #[test]
    fn color_in_a_plane_display() {
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

        assert_display!(EightBitColor(0).in_fg(), "\x1b[38;5;0m");
        assert_display!(EightBitColor(7).in_fg(), "\x1b[38;5;7m");
        assert_display!(EightBitColor(255).in_fg(), "\x1b[38;5;255m");

        assert_display!(EightBitColor(0).in_bg(), "\x1b[48;5;0m");
        assert_display!(EightBitColor(7).in_bg(), "\x1b[48;5;7m");
        assert_display!(EightBitColor(255).in_bg(), "\x1b[48;5;255m");
    }
}
