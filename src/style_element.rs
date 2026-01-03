/// An element that can be added to a [`Style`](crate::Style).
///
/// This trait is used to define elements that can be added to a `Style`. Such elements
/// include effects ([`Effect`](crate::Effect)) and colors (like [`TargetedColor`](crate::TargetedColor)).
pub trait StyleElement<S> {
    /// Adds this element to the given parameter, returning it updated.
    #[must_use]
    fn add_to(self, composed_styling: S) -> S;
}

macro_rules! impl_style_element_for {
    {$type:ty {
        args: [$self:ident, $composed_styling:ident];
        add_to: $add_to:block
    }} => {
        impl $crate::StyleElement<$crate::Style> for $type {
            fn add_to($self, $composed_styling: $crate::Style) -> $crate::Style {
                $add_to
            }
        }

        impl<C: core::fmt::Display> $crate::StyleElement<$crate::Styled<C>> for $type {
            fn add_to($self, $composed_styling: $crate::Styled<C>) -> $crate::Styled<C> {
                $add_to
            }
        }
    };
}
pub(crate) use impl_style_element_for;
