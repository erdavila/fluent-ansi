use core::fmt::{Display, Formatter, Result, Write};

use crate::{
    AppliedTo, ColorInAPlane, Effect, GetEffects, Plane, Reset, StyleAttribute, StyleElement,
    StyleSet, Styled, ToStyle, ToStyleSet,
    color::{Color, WriteColorCodes as _},
};

/// A structure representing text styling with effects and colors.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Style {
    pub(crate) effects: u16,
    pub(crate) fg: Option<Color>,
    pub(crate) bg: Option<Color>,
}

impl Style {
    /// Creates a new, empty `Style` value.
    #[must_use]
    pub const fn new() -> Self {
        Style {
            effects: 0,
            fg: None,
            bg: None,
        }
    }
}

impl ToStyleSet for Style {
    type StyleSet = Self;

    fn add(self, element: impl StyleElement) -> Self::StyleSet {
        element.add_to_style(self)
    }

    fn to_style_set(self) -> Self::StyleSet {
        self
    }
}

impl ToStyle for Style {
    fn to_style(self) -> Style {
        self
    }
}

impl AppliedTo for Style {
    fn applied_to<C: Display>(self, content: C) -> Styled<C> {
        Styled::new(content).with_style(self)
    }
}

impl StyleSet for Style {
    fn get_effects(&self) -> GetEffects<'_> {
        GetEffects {
            inner: enum_iterator::all(),
            style: self,
        }
    }

    fn set<A: StyleAttribute>(self, attr: A, value: A::Value) -> Self {
        attr.set_in_style(self, value)
    }

    fn get<A: StyleAttribute>(&self, attr: A) -> A::Value {
        attr.get_from_style(self)
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if *self == Style::new() {
            write_escape_sequence(f, 0)
        } else {
            struct Codes(Style);
            impl Display for Codes {
                fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                    let mut code_writer = CodeWriter { f, any: false };

                    for effect in enum_iterator::all::<Effect>() {
                        if self.0.get_effect(effect) {
                            code_writer.write_code(effect.get_code())?;
                        }
                    }
                    if let Some(color) = self.0.fg {
                        color.write_color_codes(Plane::Foreground, &mut code_writer)?;
                    }
                    if let Some(color) = self.0.bg {
                        color.write_color_codes(Plane::Background, &mut code_writer)?;
                    }
                    Ok(())
                }
            }
            write_escape_sequence(f, Codes(*self))
        }
    }
}

impl From<Effect> for Style {
    fn from(effect: Effect) -> Self {
        Style::new().effect(effect)
    }
}

impl From<ColorInAPlane> for Style {
    fn from(color_in_a_plane: ColorInAPlane) -> Self {
        Style::new().color(color_in_a_plane)
    }
}

impl From<Reset> for Style {
    fn from(_: Reset) -> Self {
        Style::new()
    }
}

impl PartialEq<Reset> for Style {
    fn eq(&self, other: &Reset) -> bool {
        *self == other.to_style()
    }
}

pub(crate) struct CodeWriter<'a, 'b> {
    f: &'a mut Formatter<'b>,
    any: bool,
}

impl CodeWriter<'_, '_> {
    pub(crate) fn write_code(&mut self, code: u8) -> Result {
        if self.any {
            self.f.write_char(';')?;
        }
        write!(self.f, "{code}")?;
        self.any = true;
        Ok(())
    }
}

fn write_escape_sequence(f: &mut impl Write, codes: impl Display) -> Result {
    write!(f, "\x1b[{codes}m")
}

#[cfg(test)]
mod tests {
    use crate::{
        Plane, assert_display,
        color::{BasicColor, ColorKind as _, SimpleColor},
        test_to_style_set_methods,
    };

    use super::*;

    test_to_style_set_methods!(Style::new(), Style::new());

    #[test]
    fn effect() {
        let stl = Style::new();

        assert_display!(stl, "\x1b[0m");
        assert_display!(stl.bold(), "\x1b[1m");
        assert_display!(stl.faint(), "\x1b[2m");
        assert_display!(stl.italic(), "\x1b[3m");
        assert_display!(stl.underline(), "\x1b[4m");
        assert_display!(stl.blink(), "\x1b[5m");
        assert_display!(stl.reverse(), "\x1b[7m");
        assert_display!(stl.conceal(), "\x1b[8m");
        assert_display!(stl.strikethrough(), "\x1b[9m");
        assert_display!(stl.double_underline(), "\x1b[21m");
        assert_display!(stl.overline(), "\x1b[53m");

        let bold_style = stl.bold();
        assert_eq!(bold_style.set_effect(Effect::Bold, false), stl);
        assert_eq!(bold_style.set_effect(Effect::Bold, true), stl.bold());
        assert_eq!(bold_style.set_effect(Effect::Faint, false), stl.bold());
        assert_eq!(
            bold_style.set_effect(Effect::Faint, true),
            stl.bold().faint()
        );
        assert_eq!(bold_style.get_effect(Effect::Bold), true);
        assert_eq!(bold_style.get_effect(Effect::Faint), false);
        assert_eq!(bold_style.set(Effect::Bold, false), stl);
        assert_eq!(bold_style.set(Effect::Bold, true), stl.bold());
        assert_eq!(bold_style.set(Effect::Faint, false), stl.bold());
        assert_eq!(bold_style.set(Effect::Faint, true), stl.bold().faint());
        assert_eq!(bold_style.get(Effect::Bold), true);
        assert_eq!(bold_style.get(Effect::Faint), false);
        assert_eq!(bold_style.unset(Effect::Bold), stl);
        assert_eq!(bold_style.unset(Effect::Faint), stl.bold());
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
        let stl = Style::new();
        assert_eq!(stl.get_color(Plane::Foreground), None);

        let stl = stl.fg(BasicColor::Red);
        assert_display!(stl, "\x1b[31m");
        assert_eq!(
            stl.get_color(Plane::Foreground),
            Some(Color::Simple(SimpleColor::new(BasicColor::Red)))
        );
    }

    #[test]
    fn bg() {
        let stl = Style::new();
        assert_eq!(stl.get_color(Plane::Background), None);

        let stl = stl.bg(BasicColor::Red);
        assert_display!(stl, "\x1b[41m");
        assert_eq!(
            stl.get_color(Plane::Background),
            Some(Color::Simple(SimpleColor::new(BasicColor::Red)))
        );
    }

    #[test]
    fn color() {
        let stl = Style::new();
        assert_eq!(stl.get_color(Plane::Foreground), None);
        assert_eq!(stl.get_color(Plane::Background), None);

        let stl = stl
            .set_color(Plane::Foreground, Some(BasicColor::Magenta))
            .set_color(Plane::Background, None::<Color>);
        assert_eq!(
            stl.get_color(Plane::Foreground),
            Some(Color::Simple(SimpleColor::new(BasicColor::Magenta)))
        );
        assert_eq!(stl.get_color(Plane::Background), None);

        let stl = stl
            .set_color(Plane::Foreground, None::<Color>)
            .set_color(Plane::Background, Some(BasicColor::Cyan));
        assert_eq!(stl.get_color(Plane::Foreground), None);
        assert_eq!(
            stl.get_color(Plane::Background),
            Some(Color::Simple(SimpleColor::new(BasicColor::Cyan)))
        );

        let stl = stl
            .set(Plane::Foreground, Some(BasicColor::Magenta.to_color()))
            .set(Plane::Background, None);
        assert_eq!(
            stl.get(Plane::Foreground),
            Some(BasicColor::Magenta.to_color())
        );
        assert_eq!(stl.get(Plane::Background), None);

        let stl = stl
            .set(Plane::Foreground, None)
            .set(Plane::Background, Some(BasicColor::Cyan.to_color()));
        assert_eq!(stl.get(Plane::Foreground), None);
        assert_eq!(
            stl.get(Plane::Background),
            Some(BasicColor::Cyan.to_color())
        );

        let stl = stl.unset(Plane::Background);
        assert_eq!(stl.get(Plane::Foreground), None);
        assert_eq!(stl.get(Plane::Background), None);
    }

    #[test]
    fn combined() {
        let stl = Style::new()
            .bold()
            .fg(BasicColor::Red)
            .underline()
            .bg(BasicColor::Green);
        assert_display!(stl, "\x1b[1;4;31;42m");
        assert_eq!(
            stl.unset(Effect::Bold).unset(Plane::Background),
            Style::new().underline().fg(BasicColor::Red)
        )
    }

    #[test]
    fn applied_to() {
        let stld = Style::new().bold().applied_to("CONTENT");

        assert_eq!(stld.get_content(), &"CONTENT");
        assert_eq!(stld.get_style(), Style::new().bold());
    }

    #[test]
    fn default() {
        assert_display!(Style::default(), "\x1b[0m");
    }

    #[test]
    fn to_style() {
        let stl = Style::new().bold().fg(BasicColor::Red);
        assert_eq!(stl.to_style(), stl);
    }

    #[test]
    fn from_effect() {
        assert_eq!(Style::from(Effect::Bold), Style::new().bold());
    }

    #[test]
    fn from_color_in_a_plane() {
        assert_eq!(
            Style::from(BasicColor::Red.in_fg()),
            Style::new().color(BasicColor::Red.in_fg())
        );
    }

    #[test]
    fn from_reset() {
        assert_eq!(Style::from(Reset), Style::new());
    }
}
