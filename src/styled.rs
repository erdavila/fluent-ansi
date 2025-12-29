use core::fmt::{Display, Formatter, Result};

use crate::{Effect, GetEffects, Style, StyleSet, ToStyleSet, UnderlineStyle, prelude::Color};

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

impl<C: Display> ToStyleSet for Styled<C> {
    type StyleSet = Self;

    fn to_style_set(self) -> Self::StyleSet {
        self
    }
}

impl<C: Display> StyleSet for Styled<C> {
    fn set_effect(self, effect: impl Into<Effect>, value: bool) -> Self {
        self.modify_style(|style| style.set_effect(effect, value))
    }

    fn get_effect(&self, effect: impl Into<Effect>) -> bool {
        self.style.get_effect(effect)
    }

    fn get_effects(&self) -> GetEffects {
        self.style.get_effects()
    }

    fn set_underline_style(self, underline_style: Option<UnderlineStyle>) -> Self {
        self.modify_style(|style| style.set_underline_style(underline_style))
    }

    fn get_underline_style(&self) -> Option<UnderlineStyle> {
        self.style.get_underline_style()
    }

    fn set_color(self, target: crate::ColorTarget, color: Option<impl Into<Color>>) -> Self {
        self.modify_style(|style| style.set_color(target, color))
    }

    fn get_color(&self, target: crate::ColorTarget) -> Option<Color> {
        self.style.get_color(target)
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

#[cfg(test)]
mod tests {
    use crate::{
        color::BasicColor, style_set::tests::test_style_set_methods, tests::assert_display,
        to_style_set::tests::test_to_style_set_methods,
    };

    use super::*;

    test_to_style_set_methods!(Styled::new("CONTENT"), Styled::new("CONTENT"));
    test_style_set_methods!(Styled::new("CONTENT"));

    #[test]
    fn content_and_style() {
        let stld = Styled::new("CONTENT").bold();
        assert_eq!(stld.get_content(), &"CONTENT");
        assert_eq!(stld.get_style(), Style::new().bold());

        let stld = stld.bold().with_content("NEW CONTENT");
        assert_eq!(stld.get_content(), &"NEW CONTENT");
        assert_eq!(stld.get_style(), Style::new().bold());

        let stld = stld.with_style(Style::new().fg(BasicColor::Red));
        assert_eq!(stld.get_content(), &"NEW CONTENT");
        assert_eq!(stld.get_style(), Style::new().fg(BasicColor::Red));

        let content = stld.into_content();
        assert_eq!(content, "NEW CONTENT");
    }

    #[test]
    fn effects_display() {
        let stld = Styled::new("CONTENT");

        assert_display!(stld, "CONTENT");
        assert_display!(stld.bold(), "\x1b[1mCONTENT\x1b[0m");
        assert_display!(stld.faint(), "\x1b[2mCONTENT\x1b[0m");
        assert_display!(stld.italic(), "\x1b[3mCONTENT\x1b[0m");
        assert_display!(stld.underline(), "\x1b[4mCONTENT\x1b[0m");
        assert_display!(stld.curly_underline(), "\x1b[4:3mCONTENT\x1b[0m");
        assert_display!(stld.dotted_underline(), "\x1b[4:4mCONTENT\x1b[0m");
        assert_display!(stld.dashed_underline(), "\x1b[4:5mCONTENT\x1b[0m");
        assert_display!(stld.blink(), "\x1b[5mCONTENT\x1b[0m");
        assert_display!(stld.reverse(), "\x1b[7mCONTENT\x1b[0m");
        assert_display!(stld.conceal(), "\x1b[8mCONTENT\x1b[0m");
        assert_display!(stld.strikethrough(), "\x1b[9mCONTENT\x1b[0m");
        assert_display!(stld.double_underline(), "\x1b[21mCONTENT\x1b[0m");
        assert_display!(stld.overline(), "\x1b[53mCONTENT\x1b[0m");
    }

    #[test]
    fn colors_display() {
        let stld = Styled::new("CONTENT");

        assert_display!(stld.fg(BasicColor::Red), "\x1b[31mCONTENT\x1b[0m");
        assert_display!(stld.bg(BasicColor::Red), "\x1b[41mCONTENT\x1b[0m");
    }

    #[test]
    fn combined_display() {
        let stld = Styled::new("CONTENT")
            .bold()
            .fg(BasicColor::Red)
            .underline()
            .bg(BasicColor::Green);
        assert_display!(stld, "\x1b[1;4;31;42mCONTENT\x1b[0m");
    }
}
