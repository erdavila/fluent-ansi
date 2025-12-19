use crate::{Flag, Format, Plane, ToFormatSet, color::Color};

/// A trait to represent an attribute that can be set or retrieved from a [`Format`].
pub trait FormatAttribute {
    /// The type of value associated with this attribute.
    type Value: Default;

    /// Sets this attribute in the given `Format`, returning the updated `Format`.
    #[must_use]
    fn set_in_format(self, format: Format, value: Self::Value) -> Format;

    /// Gets the value of this attribute from the given `Format`.
    #[must_use]
    fn get_from_format(self, format: &Format) -> Self::Value;
}

/// A trait to set and get formatting options on a type.
///
/// This trait extends [`ToFormatSet`] with methods to get the current state of formatting options,
/// such as checking if a flag is set or getting the color of a plane.
pub trait FormatSet: ToFormatSet<FormatSet = Self> {
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
    fn set<A: FormatAttribute>(self, attr: A, value: A::Value) -> Self;

    /// Gets the value of the given attribute.
    #[must_use]
    fn get<A: FormatAttribute>(&self, attr: A) -> A::Value;

    /// Unsets the given attribute.
    #[must_use]
    fn unset<A: FormatAttribute>(self, attr: A) -> Self {
        self.set(attr, A::Value::default())
    }
}

/// An iterator over the flags that are currently set in a [`Format`] or [`Formatted<C>`](crate::Formatted).
pub struct GetFlags<'a> {
    pub(crate) inner: enum_iterator::All<Flag>,
    pub(crate) format: &'a Format,
}
impl Iterator for GetFlags<'_> {
    type Item = Flag;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.by_ref().find(|&flag| self.format.get_flag(flag))
    }
}
