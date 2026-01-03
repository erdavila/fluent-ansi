use core::fmt::{Display, Formatter, Result};

use crate::{
    Effect, GetEffects, Style, UnderlineStyle,
    impl_macros::{composed_styling::impl_composed_styling_methods, fluent::impl_fluent_methods},
    prelude::Color,
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

    impl_fluent_methods! {
        type ComposedStyling = Styled<C>;
        args: [self];
        to_composed_styling: { self }
    }

    impl_composed_styling_methods! {
        args: [self, effect, underline_style, target, color, value];
        example_variable: r"styled";

        set_effect: {
            self.modify_style(|style| style.set_effect(effect, value))
        }

        get_effect: {
            self.style.get_effect(effect)
        }

        get_effects: {
            self.style.get_effects()
        }

        set_underline_style: {
            self.modify_style(|style| style.set_underline_style(underline_style))
        }

        get_underline_style: {
            self.style.get_underline_style()
        }

        set_color: {
            self.modify_style(|style| style.set_color(target, color))
        }

        get_color: {
            self.style.get_color(target)
        }
    }

    #[must_use]
    fn modify_style(self, f: impl FnOnce(Style) -> Style) -> Self {
        let style = f(self.style);
        Self { style, ..self }
    }
}

impl<C: Display> Display for Styled<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.style == Style::default() {
            write!(f, "{}", self.content)
        } else {
            let start = self.style;
            let end = Style::default();
            write!(f, "{start}{}{end}", self.content)
        }
    }
}
