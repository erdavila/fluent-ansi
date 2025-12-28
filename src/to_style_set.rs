use crate::{AppliedTo, ColorInAPlane, Flag, Style, StyleSet, color::Color};

/// An element that can be added to a [`Style`].
///
/// This trait is used to define elements that can be added to a `Style`. Such elements
/// include flags ([`Flag`]) and colors (like [`ColorInAPlane`]).
pub trait StyleElement: AppliedTo {
    /// Adds this element to the given `Style`, returning the updated `Style`.
    #[must_use]
    fn add_to_style(self, style: Style) -> Style;
}

/// A trait to set styling options on a type.
///
/// This trait is implemented by types that can be styled, such as [`Style`] and [`Styled`](crate::Styled).
/// It provides methods to set flags and colors, returning a type that implements [`StyleSet`].
pub trait ToStyleSet: Sized {
    /// The type that is returned by the styling methods.
    type StyleSet: StyleSet;

    /// Sets the bold flag.
    #[must_use]
    fn bold(self) -> Self::StyleSet {
        self.flag(Flag::Bold)
    }

    /// Sets the faint flag.
    #[must_use]
    fn faint(self) -> Self::StyleSet {
        self.flag(Flag::Faint)
    }

    /// Sets the italic flag.
    #[must_use]
    fn italic(self) -> Self::StyleSet {
        self.flag(Flag::Italic)
    }

    /// Sets the underline flag.
    #[must_use]
    fn underline(self) -> Self::StyleSet {
        self.flag(Flag::Underline)
    }

    /// Sets the slow blink flag.
    #[must_use]
    fn slow_blink(self) -> Self::StyleSet {
        self.flag(Flag::SlowBlink)
    }

    /// Sets the rapid blink flag.
    #[must_use]
    fn rapid_blink(self) -> Self::StyleSet {
        self.flag(Flag::RapidBlink)
    }

    /// Sets the reverse flag.
    #[must_use]
    fn reverse(self) -> Self::StyleSet {
        self.flag(Flag::Reverse)
    }

    /// Sets the conceal flag.
    #[must_use]
    fn conceal(self) -> Self::StyleSet {
        self.flag(Flag::Conceal)
    }

    /// Sets the crossed out flag.
    #[must_use]
    fn crossed_out(self) -> Self::StyleSet {
        self.flag(Flag::CrossedOut)
    }

    /// Sets the double underline flag.
    #[must_use]
    fn double_underline(self) -> Self::StyleSet {
        self.flag(Flag::DoubleUnderline)
    }

    /// Sets the overline flag.
    #[must_use]
    fn overline(self) -> Self::StyleSet {
        self.flag(Flag::Overline)
    }

    /// Sets the given flag.
    #[must_use]
    fn flag(self, flag: Flag) -> Self::StyleSet {
        self.add(flag)
    }

    /// Sets the foreground color.
    #[must_use]
    fn fg(self, color: impl Into<Color>) -> Self::StyleSet {
        self.color(ColorInAPlane::new_in_fg(color))
    }

    /// Sets the background color.
    #[must_use]
    fn bg(self, color: impl Into<Color>) -> Self::StyleSet {
        self.color(ColorInAPlane::new_in_bg(color))
    }

    /// Sets the given color in a plane.
    #[must_use]
    fn color(self, color_in_a_plane: ColorInAPlane) -> Self::StyleSet {
        self.add(color_in_a_plane)
    }

    /// Adds the given element to the style.
    #[must_use]
    fn add(self, element: impl StyleElement) -> Self::StyleSet {
        self.to_style_set().add(element)
    }

    /// Converts this value to a style set.
    #[must_use]
    fn to_style_set(self) -> Self::StyleSet;
}
