use core::fmt::{Display, Formatter, Result};

use crate::{Add, Clear as _, Format, FormatElement, Formatted, private};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
    #[must_use]
    pub fn fg(self) -> ColorInAPlane {
        self.in_plane(Plane::Foreground)
    }

    #[must_use]
    pub fn bg(self) -> ColorInAPlane {
        self.in_plane(Plane::Background)
    }

    #[must_use]
    pub fn in_plane(self, plane: Plane) -> ColorInAPlane {
        ColorInAPlane::new(self, plane)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Plane {
    Foreground,
    Background,
}

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

    #[must_use]
    pub fn to_format(self) -> Format {
        self.into()
    }
}

impl FormatElement for ColorInAPlane {
    fn add_to_format(self, format: Format) -> Format {
        format.set_color(self.plane, Some(self.color))
    }
}

impl Add for ColorInAPlane {
    type FormatSet = Format;
}

impl private::ToFormatSet<Format> for ColorInAPlane {
    fn to_format_set(self) -> Format {
        Format::new().color(self)
    }
}

impl Display for ColorInAPlane {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.to_format().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Add as _, Clear as _, Flag, assert_display};

    use super::*;

    #[test]
    fn color_fg() {
        assert_eq!(
            Color::Red.fg(),
            ColorInAPlane::new(Color::Red, Plane::Foreground)
        );
        assert_eq!(
            Color::Red.in_plane(Plane::Foreground),
            ColorInAPlane::new(Color::Red, Plane::Foreground)
        );
    }

    #[test]
    fn color_bg() {
        assert_eq!(
            Color::Red.bg(),
            ColorInAPlane::new(Color::Red, Plane::Background)
        );
        assert_eq!(
            Color::Red.in_plane(Plane::Background),
            ColorInAPlane::new(Color::Red, Plane::Background)
        );
    }

    #[test]
    fn color_in_a_plane() {
        let cp = ColorInAPlane::new(Color::Red, Plane::Foreground);

        assert_eq!(cp.get_color(), Color::Red);
        assert_eq!(cp.get_plane(), Plane::Foreground);
        assert_eq!(
            cp.to_format(),
            Format::new().set_color(Plane::Foreground, Some(Color::Red))
        );
    }

    #[test]
    fn color_in_a_plane_add_flag() {
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
