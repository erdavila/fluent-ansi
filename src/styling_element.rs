use crate::StyleSet;

/// An element that can be composed with any [styling type](crate#styling-types).
///
/// This trait is used to define elements that can be added to a `Style`. Such elements
/// include effects ([`Effect`](crate::Effect)) and colors (like [`TargetedColor`](crate::TargetedColor)).
pub trait StylingElement {
    /// Adds this element to the given `StyleSet`, returning it updated.
    #[must_use]
    fn add_to<S: StyleSet>(self, style_set: S) -> S;
}
