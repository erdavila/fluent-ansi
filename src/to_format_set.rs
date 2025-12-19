use crate::{ColorInAPlane, Flag, Format, FormatSet, color::Color};

/// An element that can be added to a [`Format`].
///
/// This trait is used to define elements that can be added to a `Format`. Such elements
/// include flags ([`Flag`]) and colors (like [`ColorInAPlane`]).
pub trait FormatElement {
    /// Adds this element to the given `Format`, returning the updated `Format`.
    #[must_use]
    fn add_to_format(self, format: Format) -> Format;
}

/// A trait to set formatting options on a type.
///
/// This trait is implemented by types that can be formatted, such as [`Format`] and [`Formatted`](crate::Formatted).
/// It provides methods to set flags and colors, returning a type that implements [`FormatSet`].
pub trait ToFormatSet: Sized {
    /// The type that is returned by the formatting methods.
    type FormatSet: FormatSet;

    /// Sets the bold flag.
    #[must_use]
    fn bold(self) -> Self::FormatSet {
        self.flag(Flag::Bold)
    }

    /// Sets the faint flag.
    #[must_use]
    fn faint(self) -> Self::FormatSet {
        self.flag(Flag::Faint)
    }

    /// Sets the italic flag.
    #[must_use]
    fn italic(self) -> Self::FormatSet {
        self.flag(Flag::Italic)
    }

    /// Sets the underline flag.
    #[must_use]
    fn underline(self) -> Self::FormatSet {
        self.flag(Flag::Underline)
    }

    /// Sets the slow blink flag.
    #[must_use]
    fn slow_blink(self) -> Self::FormatSet {
        self.flag(Flag::SlowBlink)
    }

    /// Sets the rapid blink flag.
    #[must_use]
    fn rapid_blink(self) -> Self::FormatSet {
        self.flag(Flag::RapidBlink)
    }

    /// Sets the reverse flag.
    #[must_use]
    fn reverse(self) -> Self::FormatSet {
        self.flag(Flag::Reverse)
    }

    /// Sets the conceal flag.
    #[must_use]
    fn conceal(self) -> Self::FormatSet {
        self.flag(Flag::Conceal)
    }

    /// Sets the crossed out flag.
    #[must_use]
    fn crossed_out(self) -> Self::FormatSet {
        self.flag(Flag::CrossedOut)
    }

    /// Sets the double underline flag.
    #[must_use]
    fn double_underline(self) -> Self::FormatSet {
        self.flag(Flag::DoubleUnderline)
    }

    /// Sets the overline flag.
    #[must_use]
    fn overline(self) -> Self::FormatSet {
        self.flag(Flag::Overline)
    }

    /// Sets the given flag.
    #[must_use]
    fn flag(self, flag: Flag) -> Self::FormatSet {
        self.add(flag)
    }

    /// Sets the foreground color.
    #[must_use]
    fn fg(self, color: impl Into<Color>) -> Self::FormatSet {
        self.color(ColorInAPlane::new_in_fg(color))
    }

    /// Sets the background color.
    #[must_use]
    fn bg(self, color: impl Into<Color>) -> Self::FormatSet {
        self.color(ColorInAPlane::new_in_bg(color))
    }

    /// Sets the given color in a plane.
    #[must_use]
    fn color(self, color_in_a_plane: ColorInAPlane) -> Self::FormatSet {
        self.add(color_in_a_plane)
    }

    /// Adds the given element to the formatting.
    #[must_use]
    fn add(self, element: impl FormatElement) -> Self::FormatSet {
        self.to_format_set().add(element)
    }

    /// Converts this value to a format set.
    #[must_use]
    fn to_format_set(self) -> Self::FormatSet;
}
