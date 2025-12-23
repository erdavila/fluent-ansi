use core::fmt::Result;

use crate::{
    CodeWriter, Plane,
    color::{BasicColor, WriteColorCodes},
};

/// A simple color type representing the 16 basic terminal colors (8 basic colors + bright variants).
///
/// These colors are also available as associated [`BasicColor`] constants in the [`Color`](super::Color) enum, which
/// can be turned into a [`SimpleColor`] value:
///
/// ```
/// use fluent_ansi::{prelude::*, color::{BasicColor, SimpleColor}};
///
/// assert_eq!(Color::RED.to_simple_color(), SimpleColor::new(BasicColor::Red));
/// assert_eq!(Color::RED.bright(), SimpleColor::new_bright(BasicColor::Red));
/// ```
///
/// See Wikipedia's article on [3-bit and 4-bit colors ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SimpleColor {
    basic_color: BasicColor,
    bright: bool,
}

impl SimpleColor {
    /// Creates a new simple, non-bright color.
    #[must_use]
    pub const fn new(basic_color: BasicColor) -> Self {
        Self {
            basic_color,
            bright: false,
        }
    }

    /// Creates a new bright simple color.
    #[must_use]
    pub const fn new_bright(basic_color: BasicColor) -> Self {
        Self::new(basic_color).bright()
    }

    /// Returns a bright variant of this simple color.
    #[must_use]
    pub const fn bright(self) -> Self {
        Self {
            bright: true,
            ..self
        }
    }

    /// Returns the basic color of this simple color.
    #[must_use]
    pub const fn get_basic_color(self) -> BasicColor {
        self.basic_color
    }

    /// Returns whether this simple color is bright.
    #[must_use]
    pub const fn is_bright(self) -> bool {
        self.bright
    }
}

impl WriteColorCodes for SimpleColor {
    fn write_color_codes(self, plane: Plane, writer: &mut CodeWriter) -> Result {
        let code_base = match (plane, self.bright) {
            (Plane::Foreground, false) => 30,
            (Plane::Background, false) => 40,
            (Plane::Foreground, true) => 90,
            (Plane::Background, true) => 100,
        };

        writer.write_code(code_base + self.basic_color.code_offset())
    }
}

impl From<BasicColor> for SimpleColor {
    fn from(basic_color: BasicColor) -> Self {
        SimpleColor {
            basic_color,
            bright: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_color_kind_methods;

    use super::*;

    test_color_kind_methods!(
        non_bright;
        SimpleColor::new(Color::RED),
        Color::Simple(SimpleColor {
            basic_color: BasicColor::Red,
            bright: false
        })
    );

    test_color_kind_methods!(
        bright;
        SimpleColor::new_bright(Color::RED),
        Color::Simple(SimpleColor {
            basic_color: BasicColor::Red,
            bright: true
        })
    );

    #[test]
    fn new() {
        let color = SimpleColor::new(BasicColor::Red);

        assert_eq!(
            color,
            SimpleColor {
                basic_color: BasicColor::Red,
                bright: false
            }
        );
        assert_eq!(color.get_basic_color(), BasicColor::Red);
        assert_eq!(color.is_bright(), false);
    }

    #[test]
    fn new_bright() {
        let color = SimpleColor::new_bright(BasicColor::Red);

        assert_eq!(
            color,
            SimpleColor {
                basic_color: BasicColor::Red,
                bright: true
            }
        );
        assert_eq!(color.get_basic_color(), BasicColor::Red);
        assert_eq!(color.is_bright(), true);
    }

    #[test]
    fn bright() {
        let simple_regular_color = SimpleColor::new(BasicColor::Red);
        let simple_bright_color = SimpleColor::new_bright(BasicColor::Red);

        assert_eq!(simple_regular_color.bright(), simple_bright_color);
        assert_eq!(simple_bright_color.bright(), simple_bright_color);
    }

    #[test]
    fn from_basic() {
        assert_eq!(
            SimpleColor::from(BasicColor::Red),
            SimpleColor {
                basic_color: BasicColor::Red,
                bright: false
            }
        );
    }
}
