use core::fmt::{Display, Formatter, Result};

use crate::{GetEffects, Style, StyleElement, StyleSet, ToStyleSet};

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
}
impl<C: Display> ToStyleSet for Styled<C> {
    type StyleSet = Self;

    fn add(self, element: impl StyleElement) -> Self::StyleSet {
        let style = self.style.add(element);
        self.with_style(style)
    }

    fn to_style_set(self) -> Self::StyleSet {
        self
    }
}
impl<C: Display> StyleSet for Styled<C> {
    fn get_effects(&self) -> GetEffects<'_> {
        self.style.get_effects()
    }

    fn set<A: crate::StyleAttribute>(self, attr: A, value: A::Value) -> Self {
        let style = self.style.set(attr, value);
        self.with_style(style)
    }

    fn get<A: crate::StyleAttribute>(&self, attr: A) -> A::Value {
        self.style.get(attr)
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
        Effect, Plane, assert_display,
        color::{BasicColor, Color, ColorKind as _},
        test_to_style_set_methods,
    };

    use super::*;

    test_to_style_set_methods!(Styled::new("CONTENT"), Styled::new("CONTENT"));

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
    fn display_no_style() {
        let stld = Styled::new("CONTENT");
        assert_display!(stld, "CONTENT");
    }

    #[test]
    fn add_effect() {
        let stld = Styled::new("CONTENT");

        assert_display!(stld, "CONTENT");
        assert_display!(stld.bold(), "\x1b[1mCONTENT\x1b[0m");
        assert_display!(stld.faint(), "\x1b[2mCONTENT\x1b[0m");
        assert_display!(stld.italic(), "\x1b[3mCONTENT\x1b[0m");
        assert_display!(stld.underline(), "\x1b[4mCONTENT\x1b[0m");
        assert_display!(stld.blink(), "\x1b[5mCONTENT\x1b[0m");
        assert_display!(stld.reverse(), "\x1b[7mCONTENT\x1b[0m");
        assert_display!(stld.conceal(), "\x1b[8mCONTENT\x1b[0m");
        assert_display!(stld.strikethrough(), "\x1b[9mCONTENT\x1b[0m");
        assert_display!(stld.double_underline(), "\x1b[21mCONTENT\x1b[0m");
        assert_display!(stld.overline(), "\x1b[53mCONTENT\x1b[0m");

        let bold_stld = stld.bold();
        assert_eq!(bold_stld.set_effect(Effect::Bold, false), stld);
        assert_eq!(bold_stld.set_effect(Effect::Bold, true), stld.bold());
        assert_eq!(bold_stld.set_effect(Effect::Faint, false), stld.bold());
        assert_eq!(
            bold_stld.set_effect(Effect::Faint, true),
            stld.bold().faint()
        );
        assert_eq!(bold_stld.get_effect(Effect::Bold), true);
        assert_eq!(bold_stld.get_effect(Effect::Faint), false);
        assert_eq!(bold_stld.set(Effect::Bold, false), stld);
        assert_eq!(bold_stld.set(Effect::Bold, true), stld.bold());
        assert_eq!(bold_stld.set(Effect::Faint, false), stld.bold());
        assert_eq!(bold_stld.set(Effect::Faint, true), stld.bold().faint());
        assert_eq!(bold_stld.get(Effect::Bold), true);
        assert_eq!(bold_stld.get(Effect::Faint), false);
        assert_eq!(bold_stld.unset(Effect::Bold), stld);
        assert_eq!(bold_stld.unset(Effect::Faint), stld.bold());
    }

    #[test]
    fn get_effects() {
        let style = Style::new().bold().italic().underline();
        let mut effects = style.get_effects();

        assert_eq!(effects.next(), Some(Effect::Bold));
        assert_eq!(effects.next(), Some(Effect::Italic));
        assert_eq!(effects.next(), Some(Effect::Underline));
        assert_eq!(effects.next(), None);
    }

    #[test]
    fn fg() {
        let stld = Styled::new("CONTENT");
        assert_eq!(stld.get_color(Plane::Foreground), None);

        let stld = stld.fg(BasicColor::Red);
        assert_display!(stld, "\x1b[31mCONTENT\x1b[0m");
        assert_eq!(
            stld.get_color(Plane::Foreground),
            Some(BasicColor::Red.to_color())
        );
    }

    #[test]
    fn bg() {
        let stld = Styled::new("CONTENT");
        assert_eq!(stld.get_color(Plane::Background), None);

        let stld = stld.bg(BasicColor::Red);
        assert_display!(stld, "\x1b[41mCONTENT\x1b[0m");
        assert_eq!(
            stld.get_color(Plane::Background),
            Some(BasicColor::Red.to_color())
        );
    }

    #[test]
    fn add_color() {
        let stld_base = Styled::new("CONTENT");
        assert_eq!(stld_base.get_color(Plane::Foreground), None);
        assert_eq!(stld_base.get_color(Plane::Background), None);

        let stld = stld_base
            .set_color(Plane::Foreground, Some(BasicColor::Magenta))
            .set_color(Plane::Background, None::<Color>);
        assert_eq!(
            stld.get_color(Plane::Foreground),
            Some(BasicColor::Magenta.to_color())
        );
        assert_eq!(stld.get_color(Plane::Background), None);

        let stld = stld
            .set_color(Plane::Foreground, None::<Color>)
            .set_color(Plane::Background, Some(BasicColor::Cyan));
        assert_eq!(stld.get_color(Plane::Foreground), None);
        assert_eq!(
            stld.get_color(Plane::Background),
            Some(BasicColor::Cyan.to_color())
        );

        let stld = stld
            .set(Plane::Foreground, Some(BasicColor::Magenta.to_color()))
            .set(Plane::Background, None);
        assert_eq!(
            stld.get(Plane::Foreground),
            Some(BasicColor::Magenta.to_color())
        );
        assert_eq!(stld.get(Plane::Background), None);

        let stld = stld
            .set(Plane::Foreground, None)
            .set(Plane::Background, Some(BasicColor::Cyan.to_color()));
        assert_eq!(stld.get(Plane::Foreground), None);
        assert_eq!(
            stld.get(Plane::Background),
            Some(BasicColor::Cyan.to_color())
        );

        let stld = stld.unset(Plane::Background);
        assert_eq!(stld.get(Plane::Foreground), None);
        assert_eq!(stld.get(Plane::Background), None);
    }

    #[test]
    fn combined() {
        let stld = Styled::new("CONTENT")
            .bold()
            .fg(BasicColor::Red)
            .underline()
            .bg(BasicColor::Green);
        assert_eq!(
            stld.get_style(),
            Style::new()
                .bold()
                .fg(BasicColor::Red)
                .underline()
                .bg(BasicColor::Green)
        );
        assert_display!(stld, "\x1b[1;4;31;42mCONTENT\x1b[0m");
        assert_eq!(
            stld.unset(Effect::Bold)
                .unset(Plane::Background)
                .get_style(),
            Style::new().underline().fg(BasicColor::Red)
        )
    }
}
