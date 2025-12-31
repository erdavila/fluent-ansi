use core::fmt::Result;

use crate::{CodeWriter, ColorTarget};
pub use basic::*;
pub use color::*;
pub use indexed::*;
pub use rgb::*;
pub use simple::*;

mod basic;
mod color;
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
