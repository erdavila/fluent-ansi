use crate::traits::Composed;

/// An element that can be composed with any [styling type](crate#styling-types).
///
/// This trait is used to define elements that can be added to a `Style`. Such elements
/// include effects ([`Effect`](crate::Effect)) and colors (like [`TargetedColor`](crate::TargetedColor)).
pub trait StylingElement: Copy {
    /// Adds this element to the given parameter, returning it updated.
    #[must_use]
    fn add_to<C: Composed>(self, composed: C) -> C;
}
