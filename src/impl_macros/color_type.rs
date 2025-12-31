macro_rules! impl_color_type {
    ($name:ty) => {
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

            $crate::impl_macros::applied_to::impl_applied_to!();

            /// Converts the type into a [`Style`](crate::Style).
            #[must_use]
            pub fn to_style(self) -> $crate::Style {
                $crate::TargetedColor::from(self).to_style()
            }
        }

        impl $crate::ToStyleSet for $name {
            type StyleSet = $crate::Style;

            fn to_style_set(self) -> Self::StyleSet {
                $crate::TargetedColor::from(self).to_style_set()
            }
        }

        impl $crate::StyleElement for $name {
            fn add_to<S: $crate::StyleSet>(self, style_set: S) -> S {
                $crate::TargetedColor::from(self).add_to(style_set)
            }
        }

        impl From<$name> for $crate::Style {
            fn from(value: $name) -> Self {
                value.to_style()
            }
        }
    };
}
pub(crate) use impl_color_type;
