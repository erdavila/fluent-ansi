use core::fmt::Result;

use crate::{
    CodeWriter, ColorTarget,
    color::color_methods,
    colors::{BasicColor, IndexedColor, RGBColor, SimpleColor, ToColor, WriteColorCodes},
};

/// An enum representing all supported color types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    /// A simple color (16 colors).
    Simple(SimpleColor),
    /// An 8-bit color (256 colors).
    Indexed(IndexedColor),
    /// An RGB color (24-bit/true color).
    RGB(RGBColor),
}

impl Color {
    /// Constant for the basic color black.
    pub const BLACK: BasicColor = BasicColor::Black;
    /// Constant for the basic color red.
    pub const RED: BasicColor = BasicColor::Red;
    /// Constant for the basic color green.
    pub const GREEN: BasicColor = BasicColor::Green;
    /// Constant for the basic color yellow.
    pub const YELLOW: BasicColor = BasicColor::Yellow;
    /// Constant for the basic color blue.
    pub const BLUE: BasicColor = BasicColor::Blue;
    /// Constant for the basic color magenta.
    pub const MAGENTA: BasicColor = BasicColor::Magenta;
    /// Constant for the basic color cyan.
    pub const CYAN: BasicColor = BasicColor::Cyan;
    /// Constant for the basic color white.
    pub const WHITE: BasicColor = BasicColor::White;

    /// Create an 8-bit color from the given value.
    #[must_use]
    pub const fn indexed(value: u8) -> IndexedColor {
        IndexedColor::new(value)
    }

    /// Create an RGB color from the given red, green, and blue components.
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> RGBColor {
        RGBColor::new(r, g, b)
    }

    color_methods!();

    /// Helper method to return a [`None`] value.
    ///
    /// Use it to clear the color for some target with the [`StyleSet::set_color()`](crate::StyleSet::set_color) method.
    #[must_use]
    pub const fn none() -> Option<Color> {
        None
    }

    /// Convert this color into a [`Color`].
    #[must_use]
    pub fn to_color(self) -> Color {
        self
    }
}

impl WriteColorCodes for Color {
    fn write_color_codes(self, target: ColorTarget, writer: &mut CodeWriter) -> Result {
        match self {
            Color::Simple(simple) => simple.write_color_codes(target, writer),
            Color::Indexed(indexed) => indexed.write_color_codes(target, writer),
            Color::RGB(rgb) => rgb.write_color_codes(target, writer),
        }
    }
}

impl<C: ToColor> From<C> for Color {
    fn from(value: C) -> Self {
        value.to_color()
    }
}

#[cfg(test)]
mod tests {
    use crate::colors::color_methods::tests::{
        test_color_methods, test_to_style_set_methods_with_foreground_assumed,
    };

    use super::*;

    test_color_methods!(
        simple;
        Color::Simple(SimpleColor::new(BasicColor::Red)),
        Color::Simple(SimpleColor::new(BasicColor::Red))
    );
    test_color_methods!(
        indexed;
        Color::Indexed(IndexedColor(42)),
        Color::Indexed(IndexedColor(42))
    );
    test_color_methods!(
        rgb;
        Color::RGB(RGBColor::new(0, 128, 255)),
        Color::RGB(RGBColor::new(0, 128, 255))
    );

    test_to_style_set_methods_with_foreground_assumed!(simple_fg; Color::Simple(SimpleColor::new(BasicColor::Red)));
    test_to_style_set_methods_with_foreground_assumed!(indexed_fg; Color::Indexed(IndexedColor(42)));
    test_to_style_set_methods_with_foreground_assumed!(rgb_fg; Color::RGB(RGBColor::new(0, 128, 255)));

    #[test]
    fn basic() {
        macro_rules! assert_basic_color_shortcut {
            ($shortcut:expr, $expected:expr) => {{
                // The returned type must be BasicColor instead of Color
                let color: BasicColor = $shortcut;
                assert_eq!(color, $expected);
            }};
        }

        assert_basic_color_shortcut!(Color::BLACK, BasicColor::Black);
        assert_basic_color_shortcut!(Color::RED, BasicColor::Red);
        assert_basic_color_shortcut!(Color::GREEN, BasicColor::Green);
        assert_basic_color_shortcut!(Color::YELLOW, BasicColor::Yellow);
        assert_basic_color_shortcut!(Color::BLUE, BasicColor::Blue);
        assert_basic_color_shortcut!(Color::MAGENTA, BasicColor::Magenta);
        assert_basic_color_shortcut!(Color::CYAN, BasicColor::Cyan);
        assert_basic_color_shortcut!(Color::WHITE, BasicColor::White);
    }

    #[test]
    fn indexed() {
        // The returned type must be RGBColor instead of Color
        let color: IndexedColor = Color::indexed(127);
        assert_eq!(color, IndexedColor(127));
    }

    #[test]
    fn rgb() {
        // The returned type must be RGBColor instead of Color
        let color: RGBColor = Color::rgb(0, 128, 255);
        assert_eq!(color, RGBColor::new(0, 128, 255));
    }

    #[test]
    fn from_basic_color() {
        assert_eq!(
            Color::from(BasicColor::Red),
            Color::Simple(SimpleColor::new(BasicColor::Red))
        );
    }

    #[test]
    fn from_simple_color() {
        assert_eq!(
            Color::from(SimpleColor::new(BasicColor::Red)),
            Color::Simple(SimpleColor::new(BasicColor::Red))
        );
    }

    #[test]
    fn from_indexed_color() {
        assert_eq!(
            Color::from(IndexedColor(7)),
            Color::Indexed(IndexedColor(7))
        );
    }

    #[test]
    fn from_rgb() {
        assert_eq!(
            Color::from(RGBColor::new(0, 128, 255)),
            Color::RGB(RGBColor::new(0, 128, 255))
        );
    }
}
