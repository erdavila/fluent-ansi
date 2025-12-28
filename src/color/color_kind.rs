use core::fmt::Result;

use crate::{CodeWriter, ColorTarget, TargetedColor, color::Color};

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

    /// Convert this color kind into a [`Color`].
    #[must_use]
    fn to_color(self) -> Color {
        self.into()
    }
}

pub(crate) trait WriteColorCodes: ColorKind {
    fn write_color_codes(self, target: ColorTarget, writer: &mut CodeWriter) -> Result;
}

impl<C: Into<Color>> ColorKind for C {}

#[cfg(test)]
mod tests {
    /// Includes tests for the [`ColorKind`](crate::color::ColorKind) trait methods.
    #[macro_export]
    macro_rules! test_color_kind_methods {
        ($mod:ident; $color:expr, $as_color:expr) => {
            mod $mod {
                $crate::test_color_kind_methods!($color, $as_color);
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
}
