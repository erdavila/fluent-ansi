#![warn(missing_docs)]
use crate::Style;

/// A trait to convert a type into a [`Style`].
pub trait ToStyle: Into<Style> {
    /// Converts the type into a [`Style`].
    #[must_use]
    fn to_style(self) -> Style;
}
