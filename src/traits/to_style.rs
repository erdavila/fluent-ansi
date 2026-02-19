use core::fmt::Display;

use crate::{Style, Styled};

/// Provides a method to convert a type into a [`Style`](crate::Style) and apply it to some content.
///
/// A blanket implementation of this trait is provided for all types that implement [`Into<Style>`].
pub trait ToStyle: Into<Style> + Copy {
    /// Applies the styling to the given content, returning a [`Styled<C>`](crate::Styled) instance.
    #[must_use]
    fn applied_to<C: Display>(self, content: C) -> Styled<C> {
        Styled::new(content).with_style(self.to_style())
    }

    /// Convert this type into a [`Style`](crate::Style).
    #[must_use]
    fn to_style(self) -> Style {
        self.into()
    }
}

impl<T> ToStyle for T where T: Into<Style> + Copy {}
