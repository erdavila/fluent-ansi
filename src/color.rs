use core::fmt::Result;

use crate::{BasicColor, CodeWriter, ColorInAPlane, EightBitColor, Plane};

pub(crate) mod basic;
pub(crate) mod eight_bit;

pub trait ColorKind: Into<Color> {
    #[must_use]
    fn in_fg(self) -> ColorInAPlane {
        self.in_plane(Plane::Foreground)
    }

    #[must_use]
    fn in_bg(self) -> ColorInAPlane {
        self.in_plane(Plane::Background)
    }

    #[must_use]
    fn in_plane(self, plane: Plane) -> ColorInAPlane {
        ColorInAPlane::new(self, plane)
    }

    #[must_use]
    fn to_color(self) -> Color {
        self.into()
    }
}

pub(crate) trait WriteColorCodes: ColorKind {
    fn write_color_codes(self, plane: Plane, writer: &mut CodeWriter) -> Result;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Basic(BasicColor),
    EightBit(EightBitColor),
}

impl ColorKind for Color {}

impl WriteColorCodes for Color {
    fn write_color_codes(self, plane: Plane, writer: &mut CodeWriter) -> Result {
        match self {
            Color::Basic(basic) => basic.write_color_codes(plane, writer),
            Color::EightBit(eight_bit) => eight_bit.write_color_codes(plane, writer),
        }
    }
}

impl From<BasicColor> for Color {
    fn from(basic: BasicColor) -> Self {
        Color::Basic(basic)
    }
}

impl From<EightBitColor> for Color {
    fn from(eight_bit: EightBitColor) -> Self {
        Color::EightBit(eight_bit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_fg() {
        let color = Color::Basic(BasicColor::Red);
        assert_eq!(color.in_fg(), ColorInAPlane::new(color, Plane::Foreground));
        assert_eq!(
            color.in_plane(Plane::Foreground),
            ColorInAPlane::new(color, Plane::Foreground)
        );
    }

    #[test]
    fn in_bg() {
        let color = Color::Basic(BasicColor::Red);
        assert_eq!(color.in_bg(), ColorInAPlane::new(color, Plane::Background));
        assert_eq!(
            color.in_plane(Plane::Background),
            ColorInAPlane::new(color, Plane::Background)
        );
    }

    #[test]
    fn from_basic_color() {
        assert_eq!(Color::from(BasicColor::Red), Color::Basic(BasicColor::Red));
    }

    #[test]
    fn from_eight_bit_color() {
        assert_eq!(
            Color::from(EightBitColor(7)),
            Color::EightBit(EightBitColor(7))
        );
    }
}
