use core::fmt::Result;

use crate::{
    CodeWriter, ColorTarget, Style,
    colors::{BasicColor, IndexedColor, RGBColor, SimpleColor, WriteColorCodes},
    macros::{impl_add_for_additive_type, impl_add_for_to_style_type},
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

    /// Helper method to return a [`None`] value.
    ///
    /// Use it to clear the color for some target with [`Composed::set_color()`](crate::traits::Composed::set_color).
    #[must_use]
    pub const fn none() -> Option<Color> {
        None
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

impl From<BasicColor> for Color {
    fn from(basic_color: BasicColor) -> Self {
        basic_color.to_simple_color().into()
    }
}

impl From<SimpleColor> for Color {
    fn from(simple_color: SimpleColor) -> Self {
        Color::Simple(simple_color)
    }
}

impl From<IndexedColor> for Color {
    fn from(indexed_color: IndexedColor) -> Self {
        Color::Indexed(indexed_color)
    }
}

impl From<RGBColor> for Color {
    fn from(rgb_color: RGBColor) -> Self {
        Color::RGB(rgb_color)
    }
}

impl_add_for_additive_type!(for Color, Output = Style);

impl_add_for_to_style_type!(for Color);
