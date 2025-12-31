use core::fmt::Result;

use crate::{
    CodeWriter, ColorTarget,
    color::{Color, ToColor, WriteColorCodes, impl_color},
};

/// A type alias for [`RGBColor`].
pub type RGB = RGBColor;

/// An RGB color type representing 24-bit/true color.
///
/// These colors are also available from the method [`Color::rgb()`](super::Color::rgb):
///
/// ```
/// use fluent_ansi::{prelude::*, color::RGBColor};
///
/// assert_eq!(Color::rgb(0, 128, 255), RGBColor { r: 0, g: 128, b: 255 });
/// ```
///
/// See Wikipedia's article on [24-bit colors ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code#24-bit).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RGBColor {
    /// The red component.
    pub r: u8,
    /// The green component.
    pub g: u8,
    /// The blue component.
    pub b: u8,
}

impl RGBColor {
    /// Creates a new RGB color with the given red, green, and blue components.
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl_color!(RGBColor);

impl WriteColorCodes for RGBColor {
    fn write_color_codes(self, target: ColorTarget, writer: &mut CodeWriter) -> Result {
        let target_code = match target {
            ColorTarget::Foreground => 38,
            ColorTarget::Background => 48,
            ColorTarget::Underline => 58,
        };

        writer.write_code(target_code)?;
        writer.write_code(2)?;
        writer.write_code(self.r)?;
        writer.write_code(self.g)?;
        writer.write_code(self.b)?;
        Ok(())
    }
}

impl ToColor for RGBColor {
    fn to_color(self) -> Color {
        Color::RGB(self)
    }
}
