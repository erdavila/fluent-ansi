use core::fmt::Result;

use crate::{CodeWriter, ColorTarget};
pub use basic::*;
pub use color::*;
pub(crate) use color_methods::*;
pub use indexed::*;
pub use rgb::*;
pub use simple::*;

mod basic;
mod color;
mod color_methods;
mod indexed;
mod rgb;
mod simple;

pub(crate) trait WriteColorCodes {
    fn write_color_codes(self, target: ColorTarget, writer: &mut CodeWriter) -> Result;
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

/// A trait to convert a type into a [`Color`].
pub trait ToColor: Into<Color> {
    /// Convert this type into a [`Color`].
    fn to_color(self) -> Color;
}

#[cfg(test)]
mod tests {
    use super::*;

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
