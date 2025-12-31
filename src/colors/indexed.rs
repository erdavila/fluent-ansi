use core::fmt::Result;

use crate::{
    CodeWriter, ColorTarget, color::WriteColorCodes, impl_macros::color_type::impl_color_type,
};

use super::Color;

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

impl_color_type!(IndexedColor {
    args: [self];
    to_color: { Color::Indexed(self) }
});

impl WriteColorCodes for IndexedColor {
    fn write_color_codes(self, target: ColorTarget, writer: &mut CodeWriter) -> Result {
        let target_code = match target {
            ColorTarget::Foreground => 38,
            ColorTarget::Background => 48,
            ColorTarget::Underline => 58,
        };

        writer.write_code(target_code)?;
        writer.write_code(5)?;
        writer.write_code(self.0)?;
        Ok(())
    }
}
