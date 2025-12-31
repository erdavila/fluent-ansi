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
