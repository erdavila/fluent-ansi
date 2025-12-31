use crate::{Style, StyleElement, StyleSet, TargetedColor, ToStyleSet, color::Color};

macro_rules! color_methods {
    () => {
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

        $crate::applied_to_method::applied_to_method!();

        /// Converts the type into a [`Style`](crate::Style).
        #[must_use]
        pub fn to_style(self) -> $crate::Style {
            $crate::TargetedColor::from(self).to_style()
        }
    };
}
pub(crate) use color_methods;

impl<C: Into<Color>> ToStyleSet for C {
    type StyleSet = Style;

    fn to_style_set(self) -> Self::StyleSet {
        TargetedColor::from(self).to_style_set()
    }
}

impl<C: Into<Color>> StyleElement for C {
    fn add_to<S: StyleSet>(self, style_set: S) -> S {
        TargetedColor::from(self).add_to(style_set)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    macro_rules! test_color_methods {
        ($mod:ident; $color:expr, $as_color:expr) => {
            mod $mod {
                $crate::colors::color_methods::tests::test_color_methods!($color, $as_color);
            }
        };
        ($color:expr, $as_color:expr) => {
            mod color_methods {
                use crate::{color::*, *};

                #[test]
                fn for_fg() {
                    assert_eq!(
                        $color.for_fg(),
                        TargetedColor::new($color, ColorTarget::Foreground)
                    );
                    assert_eq!(
                        $color.for_target(ColorTarget::Foreground),
                        TargetedColor::new($color, ColorTarget::Foreground)
                    );
                }

                #[test]
                fn for_bg() {
                    assert_eq!(
                        $color.for_bg(),
                        TargetedColor::new($color, ColorTarget::Background)
                    );
                    assert_eq!(
                        $color.for_target(ColorTarget::Background),
                        TargetedColor::new($color, ColorTarget::Background)
                    );
                }

                #[test]
                fn for_underline() {
                    assert_eq!(
                        $color.for_underline(),
                        TargetedColor::new($color, ColorTarget::Underline)
                    );
                    assert_eq!(
                        $color.for_target(ColorTarget::Underline),
                        TargetedColor::new($color, ColorTarget::Underline)
                    );
                }

                $crate::applied_to_method::tests::test_applied_to_method!(
                    $color,
                    Style::new().fg($color)
                );

                #[test]
                fn to_color() {
                    assert_eq!($color.to_color(), $as_color);
                }
            }
        };
    }
    pub(crate) use test_color_methods;

    /// Includes tests for the [`ToStyleSet`] trait assuming the color target is foreground.
    macro_rules! test_to_style_set_methods_with_foreground_assumed {
        ($mod:ident; $color:expr) => {
            $crate::to_style_set::tests::test_to_style_set_methods!(
                $mod;
                $color,
                Style::new().fg($color)
            );
        };
        ($color:expr) => {
            $crate::to_style_set::tests::test_to_style_set_methods!(
                $color,
                Style::new().fg($color)
            );
        };
    }
    pub(crate) use test_to_style_set_methods_with_foreground_assumed;
}
