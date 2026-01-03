/// A trait to represent an attribute that can be set or retrieved from a [`Style`](crate::Style).
pub trait StyleAttribute<S> {
    /// The type of value associated with this attribute.
    type Value: Default;

    /// Sets this attribute in the given parameter, returning it updated.
    #[must_use]
    fn set_in(self, composed_styling: S, value: Self::Value) -> S;

    /// Gets the value of this attribute from the given parameter.
    #[must_use]
    fn get_from(self, composed_styling: &S) -> Self::Value;
}

macro_rules! impl_style_atribute_for {
    {$type:ty {
        type Value = $value_type:ty;
        args: [$self:ident, $composed_styling:ident, $value:ident];
        set_in: $set_in:block
        get_from: $get_from:block
    }} => {
        impl $crate::StyleAttribute<$crate::Style> for $type {
            type Value = $value_type;

            fn set_in($self, $composed_styling: $crate::Style, $value: Self::Value) -> $crate::Style {
                $set_in
            }

            fn get_from($self, $composed_styling: &$crate::Style) -> Self::Value {
                $get_from
            }
        }

        impl<C: Display> $crate::StyleAttribute<$crate::Styled<C>> for $type {
            type Value = $value_type;

            fn set_in($self, $composed_styling: $crate::Styled<C>, $value: Self::Value) -> $crate::Styled<C> {
                $set_in
            }

            fn get_from($self, $composed_styling: &$crate::Styled<C>) -> Self::Value {
                $get_from
            }
        }
    };
}
pub(crate) use impl_style_atribute_for;
