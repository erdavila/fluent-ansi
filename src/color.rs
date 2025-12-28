//! Color types and trait.
//!
//! There are 4 color types:
//! - [`BasicColor`]: 3-bit colors with 8 variants.
//! - [`SimpleColor`]: Adds bright variants to the [`BasicColor`]s, totalling 16 colors.
//! - [`IndexedColor`]: 8-bit colors (256 colors).
//! - [`RGBColor`]: RGB colors (24-bit/true color).
//!
//! The enum [`Color`] unifies all the color types in a single type and have members to access or create colors of all types:
//!
//! ```
//! use fluent_ansi::{prelude::*, color::{BasicColor, IndexedColor, RGBColor, SimpleColor}};
//!
//! assert_eq!(Color::RED, BasicColor::Red);
//! assert_eq!(Color::RED.bright(), SimpleColor::new_bright(BasicColor::Red));
//! assert_eq!(Color::indexed(127), IndexedColor(127));
//! assert_eq!(Color::rgb(0, 128, 255), RGBColor::new(0, 128, 255));
//! ```
//!
//! All color types are convertible to [`Color`] and can be used where an `impl Into<Color>` value is expected:
//!
//! ```
//! use fluent_ansi::{prelude::*, color::{BasicColor, IndexedColor, RGBColor, SimpleColor}, ColorTarget, Style};
//!
//! let style = Style::new();
//!
//! let _ = style.fg(BasicColor::Red);
//! let _ = style.fg(SimpleColor::new_bright(BasicColor::Red));
//! let _ = style.fg(IndexedColor::new(128));
//! let _ = style.fg(RGBColor::new(0, 128, 255));
//!
//! let _ = style.bg(BasicColor::Red);
//! let _ = style.bg(SimpleColor::new_bright(BasicColor::Red));
//! let _ = style.bg(IndexedColor::new(128));
//! let _ = style.bg(RGBColor::new(0, 128, 255));
//!
//! let _ = style.set_color(ColorTarget::Foreground, Some(BasicColor::Red));
//! let _ = style.set_color(ColorTarget::Background, Some(SimpleColor::new_bright(BasicColor::Red)));
//! let _ = style.set_color(ColorTarget::Foreground, Some(IndexedColor::new(128)));
//! let _ = style.set_color(ColorTarget::Background, Some(RGBColor::new(0, 128, 255)));
//! ```
//!
//! The trait [`ColorKind`] is implemented for all color types, and provides methods to associate a color
//! to a [`ColorTarget`] (foreground or background), returning a [`TargetedColor`](crate::TargetedColor) value.

use core::fmt::Result;

use crate::{CodeWriter, ColorTarget};
pub use basic::*;
pub use color_kind::*;
pub use indexed::*;
pub use rgb::*;
pub use simple::*;

mod basic;
mod color_kind;
mod indexed;
mod rgb;
mod simple;

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
    fn from(basic: BasicColor) -> Self {
        Color::Simple(basic.to_simple_color())
    }
}

impl From<SimpleColor> for Color {
    fn from(simple: SimpleColor) -> Self {
        Color::Simple(simple)
    }
}

impl From<IndexedColor> for Color {
    fn from(indexed: IndexedColor) -> Self {
        Color::Indexed(indexed)
    }
}

impl From<RGBColor> for Color {
    fn from(rgb: RGBColor) -> Self {
        Color::RGB(rgb)
    }
}

macro_rules! impl_reflexive_partial_eq {
    ($stricter:ident :: $method:ident () -> $general:ty) => {
        impl PartialEq<$stricter> for $general {
            fn eq(&self, other: &$stricter) -> bool {
                *self == other.$method()
            }
        }

        impl PartialEq<$general> for $stricter {
            fn eq(&self, other: &$general) -> bool {
                self.$method() == *other
            }
        }
    };
}

impl_reflexive_partial_eq!(BasicColor::to_color() -> Color);
impl_reflexive_partial_eq!(SimpleColor::to_color() -> Color);
impl_reflexive_partial_eq!(IndexedColor::to_color() -> Color);
impl_reflexive_partial_eq!(RGBColor::to_color() -> Color);
impl_reflexive_partial_eq!(BasicColor::to_simple_color() -> SimpleColor);

#[cfg(test)]
mod tests {
    use crate::TargetedColor;

    use super::*;

    #[test]
    fn shortcuts() {
        assert_eq!(Color::BLACK, BasicColor::Black);
        assert_eq!(Color::RED, BasicColor::Red);
        assert_eq!(Color::GREEN, BasicColor::Green);
        assert_eq!(Color::YELLOW, BasicColor::Yellow);
        assert_eq!(Color::BLUE, BasicColor::Blue);
        assert_eq!(Color::MAGENTA, BasicColor::Magenta);
        assert_eq!(Color::CYAN, BasicColor::Cyan);
        assert_eq!(Color::WHITE, BasicColor::White);

        assert_eq!(Color::indexed(127), IndexedColor(127));

        assert_eq!(Color::rgb(0, 128, 255), RGBColor::new(0, 128, 255));
    }

    #[test]
    fn for_fg() {
        let color = BasicColor::Red.to_color();
        assert_eq!(
            color.for_fg(),
            TargetedColor::new(color, ColorTarget::Foreground)
        );
        assert_eq!(
            color.for_target(ColorTarget::Foreground),
            TargetedColor::new(color, ColorTarget::Foreground)
        );
    }

    #[test]
    fn for_bg() {
        let color = BasicColor::Red.to_color();
        assert_eq!(
            color.for_bg(),
            TargetedColor::new(color, ColorTarget::Background)
        );
        assert_eq!(
            color.for_target(ColorTarget::Background),
            TargetedColor::new(color, ColorTarget::Background)
        );
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

    #[test]
    fn eq() {
        macro_rules! assert_colors_eq {
            ($ty1:ty, $ty2:ty; $expr1:expr, $expr2:expr, ) => {{
                let color1: $ty1 = $expr1;
                let color2: $ty2 = $expr2;

                assert_eq!(
                    color1,
                    color2,
                    "{} x {}",
                    stringify!($ty1),
                    stringify!($ty2)
                );
                assert_eq!(
                    color2,
                    color1,
                    "{} x {}",
                    stringify!($ty2),
                    stringify!($ty1)
                );
            }};
        }
        macro_rules! assert_colors_ne {
            ($ty1:ty, $ty2:ty; $expr1:expr, $expr2:expr, ) => {{
                let color1: $ty1 = $expr1;
                let color2: $ty2 = $expr2;

                assert_ne!(
                    color1,
                    color2,
                    "{} x {}",
                    stringify!($ty1),
                    stringify!($ty2)
                );
                assert_ne!(
                    color2,
                    color1,
                    "{} x {}",
                    stringify!($ty2),
                    stringify!($ty1)
                );
            }};
        }

        let basic_color = BasicColor::Red;
        let simple_color = BasicColor::Red.to_simple_color();
        let indexed_color = IndexedColor::new(128);
        let rgb_color = RGBColor::new(0, 128, 255);

        let other_basic_color = BasicColor::Green;
        let other_simple_color = BasicColor::Green.to_simple_color();
        let other_indexed_color = IndexedColor::new(33);
        let other_rgb_color = RGBColor::new(33, 133, 235);

        assert_colors_eq!(Color, BasicColor;
            basic_color.to_color(),
            basic_color,
        );
        assert_colors_ne!(Color, BasicColor;
            other_basic_color.to_color(),
            basic_color,
        );
        assert_colors_ne!(Color, BasicColor;
            indexed_color.to_color(),
            basic_color,
        );

        assert_colors_eq!(Color, SimpleColor;
            simple_color.to_color(),
            simple_color,
        );
        assert_colors_ne!(Color, SimpleColor;
            other_simple_color.to_color(),
            simple_color,
        );
        assert_colors_ne!(Color, SimpleColor;
            indexed_color.to_color(),
            simple_color,
        );

        assert_colors_eq!(Color, IndexedColor;
            indexed_color.to_color(),
            indexed_color,
        );
        assert_colors_ne!(Color, IndexedColor;
            other_indexed_color.to_color(),
            indexed_color,
        );
        assert_colors_ne!(Color, IndexedColor;
            rgb_color.to_color(),
            indexed_color,
        );

        assert_colors_eq!(Color, RGBColor;
            rgb_color.to_color(),
            rgb_color,
        );
        assert_colors_ne!(Color, RGBColor;
            other_rgb_color.to_color(),
            rgb_color,
        );
        assert_colors_ne!(Color, RGBColor;
            basic_color.to_color(),
            rgb_color,
        );

        assert_colors_eq!(SimpleColor, BasicColor;
            basic_color.to_simple_color(),
            basic_color,
        );
        assert_colors_ne!(SimpleColor, BasicColor;
            other_basic_color.to_simple_color(),
            basic_color,
        );
        assert_colors_ne!(SimpleColor, BasicColor;
            basic_color.bright(),
            basic_color,
        );
    }
}
