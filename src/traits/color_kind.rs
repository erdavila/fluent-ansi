use crate::{ColorTarget, TargetedColor, color::Color};

/// A trait that provides methods for all color types.
pub trait ColorKind: Into<Color> + Copy {
    /// Associate this color with the foreground plane.
    #[must_use]
    fn for_foreground(self) -> TargetedColor {
        self.for_target(ColorTarget::Foreground)
    }

    /// Alias for [`Self::for_foreground()`].
    #[must_use]
    fn for_fg(self) -> TargetedColor {
        self.for_foreground()
    }

    /// Associate this color with the background plane.
    #[must_use]
    fn for_background(self) -> TargetedColor {
        self.for_target(ColorTarget::Background)
    }

    /// Alias for [`Self::for_background()`].
    #[must_use]
    fn for_bg(self) -> TargetedColor {
        self.for_background()
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

    /// Convert this type into a [`Color`].
    #[must_use]
    fn to_color(self) -> Color {
        self.into()
    }

    /// Converts the type into a [`TargetedColor`](crate::TargetedColor).
    #[must_use]
    fn to_targeted_color(self) -> TargetedColor {
        TargetedColor::new_for_fg(self.to_color())
    }
}

impl<T> ColorKind for T where T: Into<Color> + Copy {}
