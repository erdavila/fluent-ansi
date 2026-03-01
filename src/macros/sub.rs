macro_rules! impl_sub_for_composed_type {
    ( $( < $type_arg:ident : $bound:ident > )? for $ty:ty) => {
        impl< $( $type_arg: $bound , )? T: $crate::traits::StylingAttribute> core::ops::Sub<T> for $ty {
            type Output = Self;

            /// Removes the given attribute from the style.
            ///
            /// It delegates to the [`Composed::remove()`](crate::traits::Composed::remove) method.
            fn sub(self, rhs: T) -> Self::Output {
                $crate::traits::Composed::remove(self, rhs)
            }
        }
    };
}
pub(crate) use impl_sub_for_composed_type;
