macro_rules! impl_color_type {
    ($name:ident {
        args: [$self:ident];
    }) => {
        $crate::impl_macros::additive_styling::impl_additive_styling_type!($name {
            args: [self];
            to_style: { $crate::TargetedColor::from(self).to_style() }
        });

        $crate::impl_styling_element_for! { $name {
            args: [$self, composed_styling];
            add_to: {
                $crate::TargetedColor::from($self).add_to(composed_styling)
            }
        }}
    };
}
pub(crate) use impl_color_type;
