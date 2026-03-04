// Implements the `Add` trait for types that implement the `Additive` trait.
macro_rules! impl_add_for_additive_type {
    ( $( < $type_arg:ident : $bound:ident > )? for $ty:ty, Output = $output:ty) => {
        impl< $( $type_arg: $bound , )? T: $crate::traits::StylingElement> core::ops::Add<T> for $ty {
            type Output = $output;

            /// Adds the given element to the style.
            ///
            /// It delegates to the [`Additive::add()`](crate::traits::Additive::add) method.
            fn add(self, rhs: T) -> Self::Output {
                $crate::traits::Additive::add(self, rhs)
            }
        }

        impl< $( $type_arg: $bound )? > core::ops::Add<Style> for $ty {
            type Output = $output;

            /// Merges the styling from the given [`Style`] into this type.
            fn add(self, rhs: Style) -> Self::Output {
                let composed = $crate::traits::Additive::to_composed(self);
                $crate::traits::Composed::merge_style(composed, rhs)
            }
        }
    };
}
pub(crate) use impl_add_for_additive_type;
