use core::fmt::{Display, Formatter, Result};

use crate::{
    Style, color::Color, impl_macros::additive_styling::impl_additive_styling_type,
    impl_styling_atribute_for, impl_styling_element_for,
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
    pub fn new_for_foreground(color: impl Into<Color>) -> Self {
        Self::new(color, ColorTarget::Foreground)
    }

    /// Alias for [`TargetedColor::new_for_foreground`].
    #[must_use]
    pub fn new_for_fg(color: impl Into<Color>) -> Self {
        Self::new_for_foreground(color)
    }

    /// Creates a new color for the background plane.
    #[must_use]
    pub fn new_for_background(color: impl Into<Color>) -> Self {
        Self::new(color, ColorTarget::Background)
    }

    /// Alias for [`TargetedColor::new_for_background`].
    #[must_use]
    pub fn new_for_bg(color: impl Into<Color>) -> Self {
        Self::new_for_background(color)
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
}

impl_additive_styling_type!(TargetedColor {
    args: [self];
    to_style: { Style::new().color(self) }
});

impl_styling_element_for! { TargetedColor {
    args: [self, composed_styling];
    add_to: {
        use crate::traits::Composed as _;
        composed_styling.set_color(self.get_target(), Some(self.get_color()))
    }
}}

impl Display for TargetedColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.to_style().fmt(f)
    }
}

impl<T> From<T> for TargetedColor
where
    T: Into<Color>,
{
    fn from(value: T) -> Self {
        TargetedColor::new_for_fg(value.into())
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

impl ColorTarget {
    /// Alias for [`ColorTarget::Foreground`].
    pub const FG: Self = Self::Foreground;

    /// Alias for [`ColorTarget::Background`].
    pub const BG: Self = Self::Background;

    /// Creates a [`TargetedColor`] from a color and this color target.
    pub fn for_color(self, color: impl Into<Color>) -> TargetedColor {
        TargetedColor::new(color, self)
    }
}

impl_styling_atribute_for! { ColorTarget {
    type Value = Option<Color>;
    args: [self, composed_styling, value];

    set_in: {
        use crate::traits::Composed as _;
        composed_styling.set_color(self, value)
    }

    get_from: {
        use crate::traits::Composed as _;
        composed_styling.get_color(self)
    }
}}
