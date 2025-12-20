use crate::{Flag, Plane, Style, ToStyleSet, color::Color};

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
/// such as checking if a flag is set or getting the color of a plane.
pub trait StyleSet: ToStyleSet<StyleSet = Self> {
    /// Sets the given flag to the specified value.
    #[must_use]
    fn set_flag(self, flag: Flag, value: bool) -> Self {
        self.set(flag, value)
    }

    /// Gets whether the given flag is set.
    #[must_use]
    fn get_flag(&self, flag: Flag) -> bool {
        self.get(flag)
    }

    /// Returns an iterator over the flags that are currently set.
    #[must_use]
    fn get_flags(&self) -> GetFlags<'_>;

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

/// An iterator over the flags that are currently set in a [`Style`] or [`Styled<C>`](crate::Styled).
pub struct GetFlags<'a> {
    pub(crate) inner: enum_iterator::All<Flag>,
    pub(crate) style: &'a Style,
}
impl Iterator for GetFlags<'_> {
    type Item = Flag;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.by_ref().find(|&flag| self.style.get_flag(flag))
    }
}
