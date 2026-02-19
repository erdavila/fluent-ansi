macro_rules! impl_color_type {
    ($name:ident {
        args: [$self:ident];
    }) => {
        impl $name {
            /// Associate this color with the foreground plane.
            #[must_use]
            pub fn for_foreground(self) -> $crate::TargetedColor {
                self.for_target($crate::ColorTarget::Foreground)
            }

            /// Alias for [`Self::for_foreground()`].
            #[must_use]
            pub fn for_fg(self) -> $crate::TargetedColor {
                self.for_foreground()
            }

            /// Associate this color with the background plane.
            #[must_use]
            pub fn for_background(self) -> $crate::TargetedColor {
                self.for_target($crate::ColorTarget::Background)
            }

            /// Alias for [`Self::for_background()`].
            #[must_use]
            pub fn for_bg(self) -> $crate::TargetedColor {
                self.for_background()
            }

            /// Associate this color with the underline effect.
            #[must_use]
            pub fn for_underline(self) -> $crate::TargetedColor {
                self.for_target($crate::ColorTarget::Underline)
            }

            /// Associate this color with the specified color target.
            #[must_use]
            pub fn for_target(self, target: $crate::ColorTarget) -> $crate::TargetedColor {
                $crate::TargetedColor::new(self, target)
            }

            /// Convert this type into a [`Color`].
            #[must_use]
            pub fn to_color(self) -> Color {
                self.into()
            }
        }

        $crate::impl_macros::additive_styling::impl_additive_styling_type!($name {
            args: [self];
            to_style: { $crate::TargetedColor::from(self).to_style() }
        });

        $crate::impl_macros::from_to::impl_from_to!(
            #[doc = r"Converts the type into a [`TargetedColor`](crate::TargetedColor)"]
            fn to_targeted_color(self: $name) -> $crate::TargetedColor {
                $crate::TargetedColor::new_for_fg(self)
            }
        );

        $crate::impl_styling_element_for! { $name {
            args: [$self, composed_styling];
            add_to: {
                $crate::TargetedColor::from($self).add_to(composed_styling)
            }
        }}
    };
}
pub(crate) use impl_color_type;
