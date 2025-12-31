use crate::{
    AppliedTo, ColorTarget, Style, StyleElement, StyleSet, TargetedColor, ToStyle, ToStyleSet,
    colors::Color,
};

/// A trait for color kinds that can be converted into a [`Color`].
pub trait ColorKind: Into<Color> {
    /// Associate this color with the foreground plane.
    #[must_use]
    fn for_fg(self) -> TargetedColor {
        self.for_target(ColorTarget::Foreground)
    }

    /// Associate this color with the background plane.
    #[must_use]
    fn for_bg(self) -> TargetedColor {
        self.for_target(ColorTarget::Background)
    }

    /// Associate this color with the underline effect.
    #[must_use]
    fn for_underline(self) -> TargetedColor {
        self.for_target(ColorTarget::Underline)
    }

    /// Associate this color with the specified color target.
    #[must_use]
    fn for_target(self, target: ColorTarget) -> TargetedColor {
        TargetedColor::new(self, target)
    }
}

impl<C: Into<Color>> ColorKind for C {}

impl<CK: ColorKind> ToStyleSet for CK {
    type StyleSet = Style;

    fn to_style_set(self) -> Self::StyleSet {
        TargetedColor::from(self).to_style_set()
    }
}

impl<CK: ColorKind> ToStyle for CK {
    fn to_style(self) -> Style {
        TargetedColor::from(self).to_style()
    }
}

impl<CK: ColorKind> AppliedTo for CK {}

impl<CK: ColorKind> StyleElement for CK {
    fn add_to<S: StyleSet>(self, style_set: S) -> S {
        TargetedColor::from(self).add_to(style_set)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    /// Includes tests for the [`ColorKind`](crate::color::ColorKind) trait methods.
    macro_rules! test_color_kind_methods {
        ($mod:ident; $color:expr, $as_color:expr) => {
            mod $mod {
                $crate::colors::color_kind::tests::test_color_kind_methods!($color, $as_color);
            }
        };
        ($color:expr, $as_color:expr) => {
            mod color_kind {
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

                #[test]
                fn to_color() {
                    assert_eq!($color.to_color(), $as_color);
                }
            }
        };
    }
    pub(crate) use test_color_kind_methods;

    /// Includes tests for the [`ToStyleSet`] trait assuming the color target is foreground.
    macro_rules! test_to_style_set_methods_with_foreground_assumed {
        ($mod:ident; $color:expr) => {
            mod $mod {
                $crate::to_style_set::tests::test_to_style_set_methods!(
                    $color,
                    Style::new().fg($color)
                );
            }
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
