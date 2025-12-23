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
    use crate::{
        ColorInAPlane, Plane,
        color::{Color, ColorKind as _},
    };

    use super::*;

    #[test]
    fn indexed() {
        let color_1 = IndexedColor(7);
        assert_eq!(color_1.get_index(), 7u8);

        let color_2 = IndexedColor::new(7);
        assert_eq!(color_2.get_index(), 7u8);

        assert_eq!(color_1, color_2);
    }

    #[test]
    fn in_fg() {
        let color = IndexedColor(7);

        assert_eq!(color.in_fg(), ColorInAPlane::new(color, Plane::Foreground));
        assert_eq!(
            color.in_plane(Plane::Foreground),
            ColorInAPlane::new(color, Plane::Foreground)
        );
    }

    #[test]
    fn in_bg() {
        let color = IndexedColor(7);

        assert_eq!(color.in_bg(), ColorInAPlane::new(color, Plane::Background));
        assert_eq!(
            color.in_plane(Plane::Background),
            ColorInAPlane::new(color, Plane::Background)
        );
    }

    #[test]
    fn to_color() {
        assert_eq!(IndexedColor(7).to_color(), Color::Indexed(IndexedColor(7)));
    }
}
