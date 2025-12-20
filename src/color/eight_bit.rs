use core::fmt::Result;

use crate::{
    CodeWriter, Plane,
    color::{ColorKind, WriteColorCodes},
};

/// An 8-bit color type representing colors in the 256-color ANSI palette.
///
/// These colors are also available from the method [`Color::eight_bit()`](super::Color::eight_bit):
///
/// ```
/// use fluent_ansi::{prelude::*, color::EightBitColor};
///
/// assert_eq!(Color::eight_bit(127), EightBitColor(127));
/// ```
///
/// See Wikipedia's article on [8-bit colors ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EightBitColor(pub u8);

impl EightBitColor {
    /// Creates a new 8-bit color with the given color number (0-255).
    #[must_use]
    pub fn new(number: u8) -> Self {
        EightBitColor(number)
    }

    /// Returns the color number of this 8-bit color.
    #[must_use]
    pub fn get_number(self) -> u8 {
        self.0
    }
}

impl ColorKind for EightBitColor {}

impl WriteColorCodes for EightBitColor {
    fn write_color_codes(self, plane: Plane, writer: &mut CodeWriter) -> Result {
        let plane_code = match plane {
            Plane::Foreground => 38,
            Plane::Background => 48,
        };

        writer.write_code(plane_code)?;
        writer.write_code(5)?;
        writer.write_code(self.0)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{ColorInAPlane, Plane, color::Color};

    use super::*;

    #[test]
    fn eight_bit_color() {
        let color_1 = EightBitColor(7);
        assert_eq!(color_1.get_number(), 7u8);

        let color_2 = EightBitColor::new(7);
        assert_eq!(color_2.get_number(), 7u8);

        assert_eq!(color_1, color_2);
    }

    #[test]
    fn in_fg() {
        let color = EightBitColor(7);

        assert_eq!(color.in_fg(), ColorInAPlane::new(color, Plane::Foreground));
        assert_eq!(
            color.in_plane(Plane::Foreground),
            ColorInAPlane::new(color, Plane::Foreground)
        );
    }

    #[test]
    fn in_bg() {
        let color = EightBitColor(7);

        assert_eq!(color.in_bg(), ColorInAPlane::new(color, Plane::Background));
        assert_eq!(
            color.in_plane(Plane::Background),
            ColorInAPlane::new(color, Plane::Background)
        );
    }

    #[test]
    fn to_color() {
        assert_eq!(
            EightBitColor(7).to_color(),
            Color::EightBit(EightBitColor(7))
        );
    }
}
