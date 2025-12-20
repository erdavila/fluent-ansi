//! Color types and trait.
//!
//! There are 4 color types:
//! - [`BasicColor`]: 3-bit colors with 8 variants.
//! - [`SimpleColor`]: Adds bright variants to the [`BasicColor`]s, totalling 16 colors.
//! - [`EightBitColor`]: 8-bit colors (256 colors).
//! - [`RGBColor`]: RGB colors (24-bit/true color).
//!
//! The enum [`Color`] unifies all the color types in a single type.
//!
//! All color types are convertible to [`Color`] and can be used where an `impl Into<Color>` value is expected:
//!
//! ```
//! use fluent_ansi::{prelude::*, color::SimpleColor, Format, Plane};
//!
//! let format = Format::new();
//!
//! let _ = format.fg(BasicColor::Red);
//! let _ = format.fg(SimpleColor::new_bright(BasicColor::Red));
//! let _ = format.fg(EightBitColor::new(128));
//! let _ = format.fg(RGBColor::new(0, 128, 255));
//!
//! let _ = format.bg(BasicColor::Red);
//! let _ = format.bg(SimpleColor::new_bright(BasicColor::Red));
//! let _ = format.bg(EightBitColor::new(128));
//! let _ = format.bg(RGBColor::new(0, 128, 255));
//!
//! let _ = format.set_color(Plane::Foreground, Some(BasicColor::Red));
//! let _ = format.set_color(Plane::Background, Some(SimpleColor::new_bright(BasicColor::Red)));
//! let _ = format.set_color(Plane::Foreground, Some(EightBitColor::new(128)));
//! let _ = format.set_color(Plane::Background, Some(RGBColor::new(0, 128, 255)));
//! ```
//!
//! The trait [`ColorKind`] is implemented for all color types, and provides methods to associate a color
//! to a [`Plane`] (foreground or background), returning a [`ColorInAPlane`] value.

use core::fmt::Result;

use crate::{CodeWriter, ColorInAPlane, Plane};
pub use eight_bit::*;
pub use rgb::*;
pub use simple::*;

mod eight_bit;
mod rgb;
mod simple;

/// A trait for color kinds that can be converted into a [`Color`].
pub trait ColorKind: Into<Color> {
    /// Associate this color with the foreground plane.
    #[must_use]
    fn in_fg(self) -> ColorInAPlane {
        self.in_plane(Plane::Foreground)
    }

    /// Associate this color with the background plane.
    #[must_use]
    fn in_bg(self) -> ColorInAPlane {
        self.in_plane(Plane::Background)
    }

    /// Associate this color with the specified plane.
    #[must_use]
    fn in_plane(self, plane: Plane) -> ColorInAPlane {
        ColorInAPlane::new(self, plane)
    }

    /// Convert this color kind into a [`Color`].
    #[must_use]
    fn to_color(self) -> Color {
        self.into()
    }
}

pub(crate) trait WriteColorCodes: ColorKind {
    fn write_color_codes(self, plane: Plane, writer: &mut CodeWriter) -> Result;
}

/// An enum representing all supported color types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    /// A simple color (16 colors).
    Simple(SimpleColor),
    /// An 8-bit color (256 colors).
    EightBit(EightBitColor),
    /// An RGB color (24-bit/true color).
    RGB(RGBColor),
}

impl ColorKind for Color {}

impl WriteColorCodes for Color {
    fn write_color_codes(self, plane: Plane, writer: &mut CodeWriter) -> Result {
        match self {
            Color::Simple(simple) => simple.write_color_codes(plane, writer),
            Color::EightBit(eight_bit) => eight_bit.write_color_codes(plane, writer),
            Color::RGB(rgb) => rgb.write_color_codes(plane, writer),
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

impl From<EightBitColor> for Color {
    fn from(eight_bit: EightBitColor) -> Self {
        Color::EightBit(eight_bit)
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
impl_reflexive_partial_eq!(EightBitColor::to_color() -> Color);
impl_reflexive_partial_eq!(RGBColor::to_color() -> Color);
impl_reflexive_partial_eq!(BasicColor::to_simple_color() -> SimpleColor);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_fg() {
        let color = BasicColor::Red.to_color();
        assert_eq!(color.in_fg(), ColorInAPlane::new(color, Plane::Foreground));
        assert_eq!(
            color.in_plane(Plane::Foreground),
            ColorInAPlane::new(color, Plane::Foreground)
        );
    }

    #[test]
    fn in_bg() {
        let color = BasicColor::Red.to_color();
        assert_eq!(color.in_bg(), ColorInAPlane::new(color, Plane::Background));
        assert_eq!(
            color.in_plane(Plane::Background),
            ColorInAPlane::new(color, Plane::Background)
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
    fn from_eight_bit_color() {
        assert_eq!(
            Color::from(EightBitColor(7)),
            Color::EightBit(EightBitColor(7))
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
        let eight_bit_color = EightBitColor::new(128);
        let rgb_color = RGBColor::new(0, 128, 255);

        let other_basic_color = BasicColor::Green;
        let other_simple_color = BasicColor::Green.to_simple_color();
        let other_eight_bit_color = EightBitColor::new(33);
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
            eight_bit_color.to_color(),
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
            eight_bit_color.to_color(),
            simple_color,
        );

        assert_colors_eq!(Color, EightBitColor;
            eight_bit_color.to_color(),
            eight_bit_color,
        );
        assert_colors_ne!(Color, EightBitColor;
            other_eight_bit_color.to_color(),
            eight_bit_color,
        );
        assert_colors_ne!(Color, EightBitColor;
            rgb_color.to_color(),
            eight_bit_color,
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
