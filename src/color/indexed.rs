use core::fmt::Result;

use crate::{CodeWriter, ColorTarget, color::WriteColorCodes};

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
    /// Creates a new 8-bit color with the given color index (0-255).
    #[must_use]
    pub const fn new(index: u8) -> Self {
        IndexedColor(index)
    }

    /// Returns the color index of this 8-bit color.
    #[must_use]
    pub const fn get_index(self) -> u8 {
        self.0
    }
}

impl WriteColorCodes for IndexedColor {
    fn write_color_codes(self, target: ColorTarget, writer: &mut CodeWriter) -> Result {
        let target_code = match target {
            ColorTarget::Foreground => 38,
            ColorTarget::Background => 48,
        };

        writer.write_code(target_code)?;
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
        assert_eq!(color_1.get_index(), 7u8);

        let color_2 = IndexedColor::new(7);
        assert_eq!(color_2.get_index(), 7u8);

        assert_eq!(color_1, color_2);
    }
}
