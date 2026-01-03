/// An element that can be composed with any [styling type](crate#styling-types).
///
/// This trait is used to define elements that can be added to a `Style`. Such elements
/// include effects ([`Effect`](crate::Effect)) and colors (like [`TargetedColor`](crate::TargetedColor)).
pub trait StylingElement<S> {
    /// Adds this element to the given parameter, returning it updated.
    #[must_use]
    fn add_to(self, style_set: S) -> S;
}

macro_rules! impl_styling_element_for {
    {$type:ty {
        args: [$self:ident, $composed_styling:ident];
        add_to: $add_to:block
    }} => {
        impl $crate::StylingElement<$crate::Style> for $type {
            fn add_to($self, $composed_styling: $crate::Style) -> $crate::Style {
                $add_to
            }
        }

        impl<C: core::fmt::Display> $crate::StylingElement<$crate::Styled<C>> for $type {
            fn add_to($self, $composed_styling: $crate::Styled<C>) -> $crate::Styled<C> {
                $add_to
            }
        }
    };
}
pub(crate) use impl_styling_element_for;
