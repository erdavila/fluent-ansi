use core::fmt::{Display, Formatter, Result};

use crate::{
    Style, StyleAttribute, StyleElement, StyleSet, ToStyleSet, color::Color,
    impl_macros::applied_to::impl_applied_to,
};

/// A color in a specific color target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TargetedColor {
    color: Color,
    target: ColorTarget,
}

impl TargetedColor {
    /// Creates a new color for a specific color target.
    #[must_use]
    pub fn new(color: impl Into<Color>, target: ColorTarget) -> Self {
        let color = color.into();
        Self { color, target }
    }

    /// Creates a new color for the foreground plane.
    #[must_use]
    pub fn new_for_fg(color: impl Into<Color>) -> Self {
        Self::new(color, ColorTarget::Foreground)
    }

    /// Creates a new color for the background plane.
    #[must_use]
    pub fn new_for_bg(color: impl Into<Color>) -> Self {
        Self::new(color, ColorTarget::Background)
    }

    /// Creates a new color for the underline effects.
    #[must_use]
    pub fn new_for_underline(color: impl Into<Color>) -> Self {
        Self::new(color, ColorTarget::Underline)
    }

    /// Gets the color.
    #[must_use]
    pub const fn get_color(self) -> Color {
        self.color
    }

    /// Gets the color target.
    #[must_use]
    pub const fn get_target(self) -> ColorTarget {
        self.target
    }

    impl_applied_to!();

    /// Converts the type into a [`Style`].
    #[must_use]
    pub fn to_style(self) -> Style {
        Style::new().color(self)
    }
}

impl StyleElement for TargetedColor {
    fn add_to<S: StyleSet>(self, style_set: S) -> S {
        style_set.set_color(self.get_target(), Some(self.get_color()))
    }
}

impl ToStyleSet for TargetedColor {
    type StyleSet = Style;

    fn to_style_set(self) -> Self::StyleSet {
        self.to_style()
    }
}

impl Display for TargetedColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.to_style().fmt(f)
    }
}

impl<C: Into<Color>> From<C> for TargetedColor {
    fn from(value: C) -> Self {
        Self::new(value, ColorTarget::Foreground)
    }
}

/// The target where a color is applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorTarget {
    /// The foreground plane.
    Foreground,
    /// The background plane.
    Background,
    /// The underline effects.
    Underline,
}

impl StyleAttribute for ColorTarget {
    type Value = Option<Color>;

    fn set_in<S: StyleSet>(self, style_set: S, value: Self::Value) -> S {
        style_set.set_color(self, value)
    }

    fn get_from<S: StyleSet>(self, style_set: &S) -> Self::Value {
        style_set.get_color(self)
    }
}
