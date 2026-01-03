use core::fmt::{Display, Formatter, Result};

use crate::{
    Style, color::Color, impl_macros::fluent::impl_fluent_type, impl_style_atribute_for,
    impl_style_element_for,
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
}

impl_fluent_type!(TargetedColor {
    args: [self];
    to_style: { Style::new().color(self) }
});

impl_style_element_for! { TargetedColor {
    args: [self, composed_styling];
    add_to: {
        composed_styling.set_color(self.get_target(), Some(self.get_color()))
    }
}}

impl Display for TargetedColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.to_style().fmt(f)
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

impl_style_atribute_for! { ColorTarget {
    type Value = Option<Color>;
    args: [self, composed_styling, value];

    set_in: {
        composed_styling.set_color(self, value)
    }

    get_from: {
        composed_styling.get_color(self)
    }
}}
