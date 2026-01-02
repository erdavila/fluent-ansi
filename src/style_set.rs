use crate::{ColorTarget, Effect, GetEffects, UnderlineStyle, colors::Color};

/// A trait to represent an attribute that can be set or retrieved from a [`Style`](crate::Style).
pub trait StyleAttribute {
    /// The type of value associated with this attribute.
    type Value: Default;

    /// Sets this attribute in the given `StyleSet`, returning it updated.
    #[must_use]
    fn set_in<S: StyleSet>(self, style_set: S, value: Self::Value) -> S;

    /// Gets the value of this attribute from the given `StyleSet`.
    #[must_use]
    fn get_from<S: StyleSet>(self, style_set: &S) -> Self::Value;
}

/// A trait to set and get styling options on a type.
///
/// This trait includes methods to set and get the current state of styling options,
/// such as checking if an effect is set or getting the color of a target.
pub trait StyleSet: Sized {
    /// Sets the given effect to the specified value.
    #[must_use]
    fn set_effect(self, effect: impl Into<Effect>, value: bool) -> Self;

    /// Gets whether the given effect is set.
    #[must_use]
    fn get_effect(&self, effect: impl Into<Effect>) -> bool;

    /// Returns an iterator over the effects that are currently set.
    #[must_use]
    fn get_effects(&self) -> GetEffects;

    /// Sets the underline style.
    #[must_use]
    fn set_underline_style(self, underline_style: Option<UnderlineStyle>) -> Self;

    /// Gets the underline style.
    #[must_use]
    fn get_underline_style(&self) -> Option<UnderlineStyle>;

    /// Sets the color for the given color target.
    ///
    /// Use [`Color::none()`] to clear the color for some color target:
    ///
    /// ```
    /// # use fluent_ansi::{prelude::*, ColorTarget, Style};
    /// # let style_set = Style::new();
    /// style_set.set_color(ColorTarget::Foreground, Color::none());
    /// ```
    #[must_use]
    fn set_color(self, target: ColorTarget, color: Option<impl Into<Color>>) -> Self;

    /// Gets the color for the given color target.
    #[must_use]
    fn get_color(&self, target: ColorTarget) -> Option<Color>;

    /// Sets the given attribute to the specified value.
    #[must_use]
    fn set<A: StyleAttribute>(self, attr: A, value: A::Value) -> Self {
        attr.set_in(self, value)
    }

    /// Gets the value of the given attribute.
    #[must_use]
    fn get<A: StyleAttribute>(&self, attr: A) -> A::Value {
        attr.get_from(self)
    }

    /// Unsets the given attribute.
    #[must_use]
    fn unset<A: StyleAttribute>(self, attr: A) -> Self {
        attr.set_in(self, A::Value::default())
    }
}
