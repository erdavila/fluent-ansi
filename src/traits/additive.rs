use crate::{
    Effect, Style, TargetedColor, UnderlineEffect,
    color::Color,
    traits::{Composed, StylingElement},
};

/// A trait for types that can be converted into a composed styling type, allowing the use of additive styling methods.
pub trait Additive: Sized {
    /// The type of the composed styling that this type can be converted into.
    type Composed: Composed;

    /// Sets the bold effect.
    #[must_use]
    fn bold(self) -> Self::Composed {
        self.effect(Effect::Bold)
    }

    /// Sets the faint effect.
    #[must_use]
    fn faint(self) -> Self::Composed {
        self.effect(Effect::Faint)
    }

    /// Sets the italic effect.
    #[must_use]
    fn italic(self) -> Self::Composed {
        self.effect(Effect::Italic)
    }

    /// An alias for [`Self::solid_underline()`].
    #[must_use]
    fn underline(self) -> Self::Composed {
        self.solid_underline()
    }

    /// Sets the solid underline effect.
    #[must_use]
    fn solid_underline(self) -> Self::Composed {
        self.effect(Effect::SolidUnderline)
    }

    /// Sets the curly underline effect.
    #[must_use]
    fn curly_underline(self) -> Self::Composed {
        self.effect(Effect::CurlyUnderline)
    }

    /// Sets the dotted underline effect.
    #[must_use]
    fn dotted_underline(self) -> Self::Composed {
        self.effect(Effect::DottedUnderline)
    }

    /// Sets the dashed underline effect.
    #[must_use]
    fn dashed_underline(self) -> Self::Composed {
        self.effect(Effect::DashedUnderline)
    }

    /// Sets the blink effect.
    #[must_use]
    fn blink(self) -> Self::Composed {
        self.effect(Effect::Blink)
    }

    /// Sets the reverse effect.
    #[must_use]
    fn reverse(self) -> Self::Composed {
        self.effect(Effect::Reverse)
    }

    /// Sets the conceal effect.
    #[must_use]
    fn conceal(self) -> Self::Composed {
        self.effect(Effect::Conceal)
    }

    /// Sets the strikethrough effect.
    #[must_use]
    fn strikethrough(self) -> Self::Composed {
        self.effect(Effect::Strikethrough)
    }

    /// Sets the double underline effect.
    #[must_use]
    fn double_underline(self) -> Self::Composed {
        self.effect(Effect::DoubleUnderline)
    }

    /// Sets the overline effect.
    #[must_use]
    fn overline(self) -> Self::Composed {
        self.effect(Effect::Overline)
    }

    /// Sets the given effect.
    #[must_use]
    fn effect(self, effect: impl Into<Effect>) -> Self::Composed {
        self.to_composed().set_effect(effect, true)
    }

    /// Sets the underline effect.
    #[must_use]
    fn underline_effect(self, underline_effect: UnderlineEffect) -> Self::Composed {
        self.effect(underline_effect)
    }

    /// Sets the foreground color.
    #[must_use]
    fn foreground(self, color: impl Into<Color>) -> Self::Composed {
        self.color(TargetedColor::new_for_fg(color))
    }

    /// Alias for [`Self::foreground()`].
    #[must_use]
    fn fg(self, color: impl Into<Color>) -> Self::Composed {
        self.foreground(color)
    }

    /// Sets the background color.
    #[must_use]
    fn background(self, color: impl Into<Color>) -> Self::Composed {
        self.color(TargetedColor::new_for_bg(color))
    }

    /// Alias for [`Self::background()`].
    #[must_use]
    fn bg(self, color: impl Into<Color>) -> Self::Composed {
        self.background(color)
    }

    /// Sets the underline color.
    #[must_use]
    fn underline_color(self, color: impl Into<Color>) -> Self::Composed {
        self.color(TargetedColor::new_for_underline(color))
    }

    /// Sets the given color in a target.
    #[must_use]
    fn color(self, targeted_color: impl Into<TargetedColor>) -> Self::Composed {
        let targeted_color = targeted_color.into();
        self.to_composed().set_color(
            targeted_color.get_target(),
            Some(targeted_color.get_color()),
        )
    }

    /// Adds the given element to the style.
    #[must_use]
    fn add(self, element: impl StylingElement) -> Self::Composed {
        let composed = self.to_composed();
        element.add_to(composed)
    }

    /// Converts this type into the composed styling type, allowing the use of additive styling methods.
    #[must_use]
    fn to_composed(self) -> Self::Composed;
}

impl<T> Additive for T
where
    T: Into<Style>,
{
    type Composed = Style;

    fn to_composed(self) -> Self::Composed {
        self.into()
    }
}
