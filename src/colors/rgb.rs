use core::fmt::Result;

use crate::{
    CodeWriter, ColorTarget,
    color::{Color, ToColor, WriteColorCodes, color_methods},
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

    color_methods!();
}

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

#[cfg(test)]
mod tests {
    use crate::{
        Style, ToStyle as _, ToStyleSet as _,
        colors::color_methods::tests::{
            test_color_methods, test_to_style_set_methods_with_foreground_assumed,
        },
    };

    use super::*;

    test_color_methods!(
        RGBColor::new(0, 128, 255),
        Color::RGB(RGBColor::new(0, 128, 255))
    );

    test_to_style_set_methods_with_foreground_assumed!(RGBColor::new(0, 128, 255));

    #[test]
    fn rgb() {
        let color_1 = RGBColor {
            r: 0,
            g: 128,
            b: 255,
        };
        assert_eq!(color_1.r, 0u8);
        assert_eq!(color_1.g, 128u8);
        assert_eq!(color_1.b, 255u8);

        let color_2 = RGBColor::new(0, 128, 255);
        assert_eq!(color_2.r, 0u8);
        assert_eq!(color_2.g, 128u8);
        assert_eq!(color_2.b, 255u8);

        assert_eq!(color_1, color_2);
    }

    #[test]
    fn to_style() {
        assert_eq!(
            RGBColor::new(0, 128, 255).to_style(),
            Style::new().fg(RGBColor::new(0, 128, 255))
        );
    }
}
