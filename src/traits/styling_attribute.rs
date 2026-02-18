use crate::traits::Composed;

/// A trait to represent an attribute that can be set in or retrieved from any [composed styling value](crate#composed-styling-types).
pub trait StylingAttribute: Copy {
    /// The type of value associated with this attribute.
    type Value: Default;

    /// Sets this attribute in the given parameter, returning it updated.
    #[must_use]
    fn set_in<C: Composed>(self, composed: C, value: Self::Value) -> C;

    /// Gets the value of this attribute from the given parameter.
    #[must_use]
    fn get_from<C: Composed>(self, composed: &C) -> Self::Value;
}
