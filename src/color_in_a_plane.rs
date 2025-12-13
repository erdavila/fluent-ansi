use core::fmt::{Display, Formatter, Result};

use crate::{
    Color, Format, FormatElement, FormatSet as _, Formatted, Position, ToFormat, ToFormatSet,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColorInAPlane {
    color: Color,
    plane: Plane,
}

impl ColorInAPlane {
    #[must_use]
    pub fn new(color: Color, plane: Plane) -> Self {
        Self { color, plane }
    }

    #[must_use]
    pub fn applied_to<C: Display>(self, content: C) -> Formatted<C> {
        self.to_format().applied_to(content)
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
    use crate::{Flag, FormatSet as _, assert_display};

    use super::*;

    #[test]
    fn color_in_a_plane() {
        let cp = ColorInAPlane::new(Color::Red, Plane::Foreground);

        assert_eq!(cp.get_color(), Color::Red);
        assert_eq!(cp.get_plane(), Plane::Foreground);
        assert_eq!(
            cp.to_format_set(),
            Format::new().set_color(Plane::Foreground, Some(Color::Red))
        );
        assert_eq!(
            cp.to_format(),
            Format::new().set_color(Plane::Foreground, Some(Color::Red))
        );
    }

    #[test]
    fn flag() {
        let color_in_a_plane = Color::Red.fg();

        assert_eq!(color_in_a_plane.bold(), Format::new().fg(Color::Red).bold());
        assert_eq!(
            color_in_a_plane.flag(Flag::Bold),
            Format::new().fg(Color::Red).bold()
        );
        assert_eq!(
            color_in_a_plane.add(Flag::Bold),
            Format::new().fg(Color::Red).bold()
        );
    }
    #[test]
    fn color_in_a_plane_add_color() {
        let color_in_a_plane = Color::Red.fg();

        assert_eq!(
            color_in_a_plane.fg(Color::Green),
            Format::new().fg(Color::Green)
        );
        assert_eq!(
            color_in_a_plane.bg(Color::Green),
            Format::new().fg(Color::Red).bg(Color::Green)
        );
        assert_eq!(
            color_in_a_plane.color(Color::Green.bg()),
            Format::new().fg(Color::Red).bg(Color::Green)
        );
        assert_eq!(
            color_in_a_plane.add(Color::Green.bg()),
            Format::new().fg(Color::Red).bg(Color::Green)
        );
    }

    #[test]
    fn color_in_a_plane_applied_to() {
        let fmtd = Color::Red.fg().applied_to("CONTENT");

        assert_eq!(fmtd.get_content(), &"CONTENT");
        assert_eq!(fmtd.get_format(), Format::new().fg(Color::Red));
    }

    #[test]
    fn color_in_a_plane_to_format() {
        assert_eq!(Color::Red.fg().to_format(), Format::new().fg(Color::Red));
        assert_eq!(
            Color::Green.bg().to_format(),
            Format::new().bg(Color::Green)
        );
    }

    #[test]
    fn color_in_a_plane_display() {
        assert_display!(Color::Black.fg(), "\x1b[30m");
        assert_display!(Color::Red.fg(), "\x1b[31m");
        assert_display!(Color::Green.fg(), "\x1b[32m");
        assert_display!(Color::Yellow.fg(), "\x1b[33m");
        assert_display!(Color::Blue.fg(), "\x1b[34m");
        assert_display!(Color::Magenta.fg(), "\x1b[35m");
        assert_display!(Color::Cyan.fg(), "\x1b[36m");
        assert_display!(Color::White.fg(), "\x1b[37m");

        assert_display!(Color::Black.bg(), "\x1b[40m");
        assert_display!(Color::Red.bg(), "\x1b[41m");
        assert_display!(Color::Green.bg(), "\x1b[42m");
        assert_display!(Color::Yellow.bg(), "\x1b[43m");
        assert_display!(Color::Blue.bg(), "\x1b[44m");
        assert_display!(Color::Magenta.bg(), "\x1b[45m");
        assert_display!(Color::Cyan.bg(), "\x1b[46m");
        assert_display!(Color::White.bg(), "\x1b[47m");
    }
}
