use crate::StyleSet;

/// An element that can be added to a [`Style`](crate::Style).
///
/// This trait is used to define elements that can be added to a `Style`. Such elements
/// include effects ([`Effect`](crate::Effect)) and colors (like [`TargetedColor`](crate::TargetedColor)).
pub trait StyleElement {
    /// Adds this element to the given `StyleSet`, returning it updated.
    #[must_use]
    fn add_to<S: StyleSet>(self, style_set: S) -> S;
}
