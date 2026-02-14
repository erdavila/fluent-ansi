use core::fmt::Display;

use crate::{Style, Styled};

/// A trait for types that can be styled using a fluent API, returning a [`Styled`] value.
///
/// This trait is implemented for all types that implement [`Display`].
pub trait ToStyled: Display + Sized {
    /// Converts the value into a [`Styled`] value with the same content and an empty style.
    fn styled(self) -> Styled<Self> {
        Styled::new(self)
    }

    /// Returns a new [`Styled`] value with the same content and the given style.
    fn with_style(self, style: Style) -> Styled<Self> {
        self.styled().with_style(style)
    }
}

impl<T> ToStyled for T where T: Display {}
