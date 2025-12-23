use crate::{Effect, Plane, Style, ToStyleSet, color::Color};

/// A trait to represent an attribute that can be set or retrieved from a [`Style`].
pub trait StyleAttribute {
    /// The type of value associated with this attribute.
    type Value: Default;

    /// Sets this attribute in the given `Style`, returning the updated `Style`.
    #[must_use]
    fn set_in_style(self, style: Style, value: Self::Value) -> Style;

    /// Gets the value of this attribute from the given `Style`.
    #[must_use]
    fn get_from_style(self, style: &Style) -> Self::Value;
}

/// A trait to set and get styling options on a type.
///
/// This trait extends [`ToStyleSet`] with methods to get the current state of styling options,
/// such as checking if an effect is set or getting the color of a plane.
pub trait StyleSet: ToStyleSet<StyleSet = Self> {
    /// Sets the given effect to the specified value.
    #[must_use]
    fn set_effect(self, effect: Effect, value: bool) -> Self {
        self.set(effect, value)
    }

    /// Gets whether the given effect is set.
    #[must_use]
    fn get_effect(&self, effect: Effect) -> bool {
        self.get(effect)
    }

    /// Returns an iterator over the effects that are currently set.
    #[must_use]
    fn get_effects(&self) -> GetEffects<'_>;

    /// Sets the color for the given plane (foreground or background).
    #[must_use]
    fn set_color(self, plane: Plane, color: Option<impl Into<Color>>) -> Self {
        let color: Option<Color> = color.map(Into::into);
        self.set(plane, color)
    }

    /// Gets the color for the given plane (foreground or background).
    #[must_use]
    fn get_color(&self, plane: Plane) -> Option<Color> {
        self.get(plane)
    }

    /// Sets the given attribute to the specified value.
    #[must_use]
    fn set<A: StyleAttribute>(self, attr: A, value: A::Value) -> Self;

    /// Gets the value of the given attribute.
    #[must_use]
    fn get<A: StyleAttribute>(&self, attr: A) -> A::Value;

    /// Unsets the given attribute.
    #[must_use]
    fn unset<A: StyleAttribute>(self, attr: A) -> Self {
        self.set(attr, A::Value::default())
    }
}

/// An iterator over the effects that are currently set in a [`Style`] or [`Styled<C>`](crate::Styled).
pub struct GetEffects<'a> {
    pub(crate) inner: enum_iterator::All<Effect>,
    pub(crate) style: &'a Style,
}
impl Iterator for GetEffects<'_> {
    type Item = Effect;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .by_ref()
            .find(|&effect| self.style.get_effect(effect))
    }
}
