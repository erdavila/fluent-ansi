use core::fmt::{Display, Formatter, Result};

use crate::Format;

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

impl Display for ColorInAPlane {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.to_format().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{WithFormat as _, assert_display};

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
            Format::new().with_color(Some(Color::Red), Plane::Foreground)
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
