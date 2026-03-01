use core::fmt::{Display, Formatter, Result};

use crate::{
    ColorTarget, Effect, GetEffects, Style, UnderlineEffect,
    macros::{impl_add_for_additive_type, impl_sub_for_composed_type},
    prelude::Color,
    traits::{Additive, Composed},
};

/// A value that associates some content with a specific style.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Styled<C: Display> {
    content: C,
    style: Style,
}

impl<C: Display> Styled<C> {
    /// Creates a new `Styled<C>` value with the given content and empty style.
    #[must_use]
    pub const fn new(content: C) -> Self {
        Self {
            content,
            style: Style::new(),
        }
    }

    /// Gets a reference to the enclosed content.
    #[must_use]
    pub const fn get_content(&self) -> &C {
        &self.content
    }

    /// Returns a new `Styled<C2>` value with the same style and the given content.
    #[must_use]
    pub const fn with_content<C2: Display>(&self, content: C2) -> Styled<C2> {
        Styled {
            content,
            style: self.style,
        }
    }

    /// Consumes the `Styled<C>` value and returns the enclosed content.
    #[must_use]
    pub fn into_content(self) -> C {
        self.content
    }

    /// Gets the current style.
    #[must_use]
    pub const fn get_style(&self) -> Style {
        self.style
    }

    /// Returns a new `Styled<C>` value with the same content and the given style.
    #[must_use]
    pub fn with_style(self, style: Style) -> Styled<C> {
        Self { style, ..self }
    }

    #[must_use]
    fn modify_style(self, f: impl FnOnce(Style) -> Style) -> Self {
        let style = f(self.style);
        Self { style, ..self }
    }
}

impl<C: Display> Additive for Styled<C> {
    type Composed = Self;

    fn to_composed(self) -> Self::Composed {
        self
    }
}

impl<C: Display> Composed for Styled<C> {
    fn set_effect(self, effect: impl Into<Effect>, value: bool) -> Self {
        self.modify_style(|style| style.set_effect(effect, value))
    }

    fn get_effect(&self, effect: impl Into<Effect>) -> bool {
        self.style.get_effect(effect)
    }

    fn get_effects(&self) -> GetEffects {
        self.style.get_effects()
    }

    fn set_underline_effect(self, underline_effect: Option<UnderlineEffect>) -> Self {
        self.modify_style(|style| style.set_underline_effect(underline_effect))
    }

    fn get_underline_effect(&self) -> Option<UnderlineEffect> {
        self.style.get_underline_effect()
    }

    fn set_color(self, target: ColorTarget, color: Option<impl Into<Color>>) -> Self {
        self.modify_style(|style| style.set_color(target, color))
    }

    fn get_color(&self, target: ColorTarget) -> Option<Color> {
        self.style.get_color(target)
    }

    fn merge_style(self, other: Style) -> Self {
        self.modify_style(|style| style.merge_style(other))
    }

    fn set_enabled(self, enabled: bool) -> Self {
        self.modify_style(|style| style.set_enabled(enabled))
    }

    fn is_enabled(&self) -> bool {
        self.style.is_enabled()
    }
}

impl<C: Display> Display for Styled<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.style == Style::default() {
            write!(f, "{}", self.content)
        } else {
            let start = self.style;
            let end = Style::default().set_enabled(self.style.is_enabled());
            write!(f, "{start}{}{end}", self.content)
        }
    }
}

impl_add_for_additive_type!(<C: Display> for Styled<C>, Output = Styled<C>);

impl_sub_for_composed_type!(<C: Display> for Styled<C>);
