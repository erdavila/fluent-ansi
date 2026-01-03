macro_rules! impl_color_type {
    ($name:ident {
        args: [$self:ident];
        to_color: $to_color:tt
    }) => {
        impl $name {
            /// Associate this color with the foreground plane.
            #[must_use]
            pub fn for_fg(self) -> $crate::TargetedColor {
                self.for_target($crate::ColorTarget::Foreground)
            }

            /// Associate this color with the background plane.
            #[must_use]
            pub fn for_bg(self) -> $crate::TargetedColor {
                self.for_target($crate::ColorTarget::Background)
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
        }

        $crate::impl_macros::fluent::impl_fluent_type!($name {
            args: [self];
            to_style: { $crate::TargetedColor::from(self).to_style() }
        });

        $crate::impl_macros::color_type::__impl_color_type__to_color!($name, $self, $to_color);

        $crate::impl_macros::from_to::impl_from_to!(
            #[doc = r"Converts the type into a [`TargetedColor`](crate::TargetedColor)"]
            fn to_targeted_color(self: $name) -> $crate::TargetedColor {
                $crate::TargetedColor::new_for_fg(self)
            }
        );

        $crate::impl_style_element_for! { $name {
            args: [$self, composed_styling];
            add_to: {
                $crate::TargetedColor::from($self).add_to(composed_styling)
            }
        }}
    };
}
pub(crate) use impl_color_type;

macro_rules! __impl_color_type__to_color {
    ($name:ident, $self:ident, SELF) => {
        // Defines only the to_color method
        impl $name {
            #[doc = r"Convert this type into a [`Color`]."]
            #[must_use]
            pub fn to_color(self) -> Color {
                self
            }
        }
    };

    ($name:ident, $self:ident, $to_color:block) => {
        // Defines the to_color method and impl From<$name> for Color
        $crate::impl_macros::from_to::impl_from_to!(
            #[doc = r"Convert this type into a [`Color`]."]
            fn to_color($self: $name) -> Color {
                $to_color
            }
        );
    };
}
pub(crate) use __impl_color_type__to_color;
