use crate::{Effect, StyleSet, TargetedColor, UnderlineStyle, colors::Color};

/// An element that can be added to a [`Style`](crate::Style).
///
/// This trait is used to define elements that can be added to a `Style`. Such elements
/// include effects ([`Effect`]) and colors (like [`TargetedColor`]).
pub trait StyleElement {
    /// Adds this element to the given `StyleSet`, returning it updated.
    #[must_use]
    fn add_to<S: StyleSet>(self, style_set: S) -> S;
}

/// A trait to set styling options on a type.
///
/// This trait is implemented by types that can be styled, such as [`Style`](crate::Style) and [`Styled`](crate::Styled).
/// It provides methods to set effects and colors, returning a type that implements [`StyleSet`].
pub trait ToStyleSet: Sized {
    /// The type that is returned by the styling methods.
    type StyleSet: StyleSet;

    /// Sets the bold effect.
    #[must_use]
    fn bold(self) -> Self::StyleSet {
        self.effect(Effect::Bold)
    }

    /// Sets the faint effect.
    #[must_use]
    fn faint(self) -> Self::StyleSet {
        self.effect(Effect::Faint)
    }

    /// Sets the italic effect.
    #[must_use]
    fn italic(self) -> Self::StyleSet {
        self.effect(Effect::Italic)
    }

    /// Sets the solid underline effect.
    #[must_use]
    fn underline(self) -> Self::StyleSet {
        self.effect(Effect::Underline)
    }

    /// Sets the curly underline effect.
    #[must_use]
    fn curly_underline(self) -> Self::StyleSet {
        self.effect(Effect::CurlyUnderline)
    }

    /// Sets the dotted underline effect.
    #[must_use]
    fn dotted_underline(self) -> Self::StyleSet {
        self.effect(Effect::DottedUnderline)
    }

    /// Sets the dashed underline effect.
    #[must_use]
    fn dashed_underline(self) -> Self::StyleSet {
        self.effect(Effect::DashedUnderline)
    }

    /// Sets the blink effect.
    #[must_use]
    fn blink(self) -> Self::StyleSet {
        self.effect(Effect::Blink)
    }

    /// Sets the reverse effect.
    #[must_use]
    fn reverse(self) -> Self::StyleSet {
        self.effect(Effect::Reverse)
    }

    /// Sets the conceal effect.
    #[must_use]
    fn conceal(self) -> Self::StyleSet {
        self.effect(Effect::Conceal)
    }

    /// Sets the strikethrough effect.
    #[must_use]
    fn strikethrough(self) -> Self::StyleSet {
        self.effect(Effect::Strikethrough)
    }

    /// Sets the double underline effect.
    #[must_use]
    fn double_underline(self) -> Self::StyleSet {
        self.effect(Effect::DoubleUnderline)
    }

    /// Sets the overline effect.
    #[must_use]
    fn overline(self) -> Self::StyleSet {
        self.effect(Effect::Overline)
    }

    /// Sets the given effect.
    #[must_use]
    fn effect(self, effect: impl Into<Effect>) -> Self::StyleSet {
        self.to_style_set().set_effect(effect, true)
    }

    /// Sets the underline style.
    #[must_use]
    fn underline_style(self, underline_style: UnderlineStyle) -> Self::StyleSet {
        self.effect(underline_style)
    }

    /// Sets the foreground color.
    #[must_use]
    fn fg(self, color: impl Into<Color>) -> Self::StyleSet {
        self.color(TargetedColor::new_for_fg(color))
    }

    /// Sets the background color.
    #[must_use]
    fn bg(self, color: impl Into<Color>) -> Self::StyleSet {
        self.color(TargetedColor::new_for_bg(color))
    }

    /// Sets the underline color.
    #[must_use]
    fn underline_color(self, color: impl Into<Color>) -> Self::StyleSet {
        self.color(TargetedColor::new_for_underline(color))
    }

    /// Sets the given color in a target.
    #[must_use]
    fn color(self, targeted_color: impl Into<TargetedColor>) -> Self::StyleSet {
        let targeted_color = targeted_color.into();
        self.to_style_set().set_color(
            targeted_color.get_target(),
            Some(targeted_color.get_color()),
        )
    }

    /// Adds the given element to the style.
    #[must_use]
    fn add(self, element: impl StyleElement) -> Self::StyleSet {
        let style_set = self.to_style_set();
        element.add_to(style_set)
    }

    /// Converts this value to a style set.
    #[must_use]
    fn to_style_set(self) -> Self::StyleSet;
}
