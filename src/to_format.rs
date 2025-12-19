#![warn(missing_docs)]
use crate::Format;

/// A trait to convert a type into a [`Format`].
pub trait ToFormat: Into<Format> {
    /// Converts the type into a [`Format`].
    #[must_use]
    fn to_format(self) -> Format;
}
