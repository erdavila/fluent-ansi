use core::fmt::Result;

use crate::{CodeWriter, ColorKind, Plane, WriteColorCodes};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BasicColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}
impl BasicColor {
    fn code_offset(self) -> u8 {
        self as u8
    }
}

impl ColorKind for BasicColor {}

impl WriteColorCodes for BasicColor {
    fn write_color_codes(self, plane: Plane, writer: &mut CodeWriter) -> Result {
        let code_base = match plane {
            Plane::Foreground => 30,
            Plane::Background => 40,
        };

        writer.write_code(code_base + self.code_offset())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Color, ColorInAPlane, Plane};

    use super::*;

    #[test]
    fn in_fg() {
        assert_eq!(
            BasicColor::Red.in_fg(),
            ColorInAPlane::new(BasicColor::Red, Plane::Foreground)
        );
        assert_eq!(
            BasicColor::Red.in_plane(Plane::Foreground),
            ColorInAPlane::new(BasicColor::Red, Plane::Foreground)
        );
    }

    #[test]
    fn in_bg() {
        assert_eq!(
            BasicColor::Red.in_bg(),
            ColorInAPlane::new(BasicColor::Red, Plane::Background)
        );
        assert_eq!(
            BasicColor::Red.in_plane(Plane::Background),
            ColorInAPlane::new(BasicColor::Red, Plane::Background)
        );
    }

    #[test]
    fn to_color() {
        assert_eq!(BasicColor::Red.to_color(), Color::Basic(BasicColor::Red));
    }
}
