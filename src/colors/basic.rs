use crate::{
    color::color_methods,
    colors::{Color, SimpleColor, ToColor},
};

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
    /// Returns a bright variant of this basic color.
    #[must_use]
    pub fn bright(self) -> SimpleColor {
        SimpleColor::new_bright(self)
    }

    #[must_use]
    pub(crate) fn code_offset(self) -> u8 {
        self as u8
    }

    color_methods!();

    /// Convert this basic color into a [`SimpleColor`].
    #[must_use]
    pub fn to_simple_color(self) -> SimpleColor {
        SimpleColor::new(self)
    }
}

impl ToColor for BasicColor {
    /// Convert this basic color into a [`Color`].
    fn to_color(self) -> Color {
        self.to_simple_color().to_color()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Style, ToStyle as _, ToStyleSet as _,
        colors::{
            BasicColor,
            color_methods::tests::{
                test_color_methods, test_to_style_set_methods_with_foreground_assumed,
            },
        },
    };

    use super::*;

    test_color_methods!(
        BasicColor::Red,
        Color::Simple(SimpleColor::new(BasicColor::Red))
    );

    test_to_style_set_methods_with_foreground_assumed!(BasicColor::Red);

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

    #[test]
    fn to_style() {
        assert_eq!(BasicColor::Red.to_style(), Style::new().fg(BasicColor::Red));
    }
}
