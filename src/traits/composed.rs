use crate::{
    ColorTarget, Effect, GetEffects, Style, StylingAttribute, UnderlineEffect, color::Color,
};

/// A trait for types that are composed of styling.
///
/// This trait provides methods to set, query, and unset styling.
pub trait Composed {
    /// Sets the given effect to the specified value.
    #[must_use]
    fn set_effect(self, effect: impl Into<Effect>, value: bool) -> Self;

    /// Gets whether the given effect is set.
    #[must_use]
    fn get_effect(&self, effect: impl Into<Effect>) -> bool;

    /// Returns an iterator over the effects that are currently set.
    #[must_use]
    fn get_effects(&self) -> GetEffects;

    /// Sets the underline effect.
    #[must_use]
    fn set_underline_effect(self, underline_effect: Option<UnderlineEffect>) -> Self;

    /// Gets the underline effect.
    #[must_use]
    fn get_underline_effect(&self) -> Option<UnderlineEffect>;

    /// Sets the color for the given color target.
    ///
    /// To clear the color for some color target, the color type must be specified in the `None` value.
    /// To help with that, the [`Color::none()`](Color::none) method can be used:
    ///
    /// ```
    /// # use fluent_ansi::{prelude::*, ColorTarget, Style};
    /// # let composed = Style::new();
    /// composed.set_color(ColorTarget::Foreground, None::<Color>);
    /// // or
    /// composed.set_color(ColorTarget::Foreground, Color::none());
    /// ```
    #[must_use]
    fn set_color(self, target: ColorTarget, color: Option<impl Into<Color>>) -> Self;

    /// Gets the color for the given color target.
    #[must_use]
    fn get_color(&self, target: ColorTarget) -> Option<Color>;

    /// Sets the given attribute to the specified value.
    #[must_use]
    fn set<A: StylingAttribute<Self>>(self, attr: A, value: A::Value) -> Self
    where
        Self: Sized,
    {
        attr.set_in(self, value)
    }

    /// Gets the value of the given attribute.
    #[must_use]
    fn get<A: StylingAttribute<Self>>(&self, attr: A) -> A::Value
    where
        Self: Sized,
    {
        attr.get_from(self)
    }

    /// Clears the given attribute.
    #[must_use]
    fn remove<A: StylingAttribute<Self>>(self, attr: A) -> Self
    where
        Self: Sized,
    {
        attr.set_in(self, A::Value::default())
    }

    /// Merge styling from the `Style` argument.
    #[must_use]
    fn merge_style(self, other: Style) -> Self;

    /// Sets whether the styling is enabled.
    #[must_use]
    fn set_enabled(self, enabled: bool) -> Self;

    /// Gets whether the styling is enabled.
    #[must_use]
    fn is_enabled(&self) -> bool;
}
