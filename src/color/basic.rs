use crate::color::SimpleColor;

/// The 8 basic non-bright terminal colors.
///
/// These colors are also available as associated constants in the [`Color`](super::Color) enum:
///
/// ```
/// use fluent_ansi::{prelude::*, color::BasicColor};
///
/// assert_eq!(Color::RED, BasicColor::Red);
/// assert_eq!(Color::GREEN, BasicColor::Green);
/// assert_eq!(Color::BLUE, BasicColor::Blue);
/// // etc.
/// ```
///
/// See Wikipedia's article on [3-bit and 4-bit colors ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BasicColor {
    /// The black color.
    Black,
    /// The red color.
    Red,
    /// The green color.
    Green,
    /// The yellow color.
    Yellow,
    /// The blue color.
    Blue,
    /// The magenta color.
    Magenta,
    /// The cyan color.
    Cyan,
    /// The white color.
    White,
}

impl BasicColor {
    /// Convert this basic color into a [`SimpleColor`].
    #[must_use]
    pub fn to_simple_color(self) -> SimpleColor {
        self.into()
    }

    /// Returns a bright variant of this basic color.
    #[must_use]
    pub fn bright(self) -> SimpleColor {
        SimpleColor::new_bright(self)
    }

    #[must_use]
    pub(crate) fn code_offset(self) -> u8 {
        self as u8
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::BasicColor, test_color_kind_methods};

    use super::*;

    test_color_kind_methods!(
        BasicColor::Red,
        Color::Simple(SimpleColor::new(BasicColor::Red))
    );

    #[test]
    fn bright() {
        assert_eq!(
            BasicColor::Red.bright(),
            SimpleColor::new_bright(BasicColor::Red)
        );
    }

    #[test]
    fn to_simple_color() {
        assert_eq!(
            BasicColor::Red.to_simple_color(),
            SimpleColor::new(BasicColor::Red)
        );
    }
}
