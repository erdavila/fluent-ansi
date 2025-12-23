use core::fmt::Result;

use crate::{CodeWriter, Plane, color::WriteColorCodes};

/// An 8-bit color type representing colors in the 256-color ANSI palette.
///
/// These colors are also available from the method [`Color::indexed()`](super::Color::indexed):
///
/// ```
/// use fluent_ansi::{prelude::*, color::IndexedColor};
///
/// assert_eq!(Color::indexed(127), IndexedColor(127));
/// ```
///
/// See Wikipedia's article on [8-bit colors ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndexedColor(pub u8);

impl IndexedColor {
    /// Creates a new 8-bit color with the given color number (0-255).
    #[must_use]
    pub const fn new(number: u8) -> Self {
        IndexedColor(number)
    }

    /// Returns the color number of this 8-bit color.
    #[must_use]
    pub const fn get_number(self) -> u8 {
        self.0
    }
}

impl WriteColorCodes for IndexedColor {
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
    use crate::test_color_kind_methods;

    use super::*;

    test_color_kind_methods!(IndexedColor(7), Color::Indexed(IndexedColor(7)));

    #[test]
    fn indexed() {
        let color_1 = IndexedColor(7);
        assert_eq!(color_1.get_number(), 7u8);

        let color_2 = IndexedColor::new(7);
        assert_eq!(color_2.get_number(), 7u8);

        assert_eq!(color_1, color_2);
    }
}
