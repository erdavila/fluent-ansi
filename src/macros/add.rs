macro_rules! impl_add_styling_element {
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
    };
}
pub(crate) use impl_add_styling_element;
