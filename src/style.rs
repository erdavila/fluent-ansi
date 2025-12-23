use core::fmt::{Display, Formatter, Result, Write};

use crate::{
    AppliedTo, ColorInAPlane, Effect, Plane, Reset, StyleAttribute, StyleElement, StyleSet, Styled,
    ToStyle, ToStyleSet, UnderlineStyle,
    color::{Color, WriteColorCodes as _},
    style::encoded_effects::EncodedEffects,
};

pub use encoded_effects::*;

mod encoded_effects;

/// A structure representing text styling with effects and colors.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Style {
    pub(crate) encoded_effects: EncodedEffects,
    pub(crate) fg: Option<Color>,
    pub(crate) bg: Option<Color>,
}

impl Style {
    /// Creates a new, empty `Style` value.
    #[must_use]
    pub const fn new() -> Self {
        Style {
            encoded_effects: EncodedEffects::new(),
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
    fn get_effects(&self) -> GetEffects {
        self.encoded_effects.get_effects()
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

                    for effect in Effect::all() {
                        if self.0.get_effect(effect) {
                            effect.write_codes(&mut code_writer)?;
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

impl From<UnderlineStyle> for Style {
    fn from(underline_style: UnderlineStyle) -> Self {
        Style::new().underline_style(underline_style)
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
    pub(crate) fn write_code(&mut self, code: impl Display) -> Result {
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
        assert_display,
        color::{BasicColor, ColorKind as _},
        test_style_set_methods, test_to_style_set_methods,
    };

    use super::*;

    test_to_style_set_methods!(Style::new(), Style::new());
    test_style_set_methods!(Style::new());

    #[test]
    fn effects_display() {
        let stl = Style::new();

        assert_display!(stl, "\x1b[0m");
        assert_display!(stl.bold(), "\x1b[1m");
        assert_display!(stl.faint(), "\x1b[2m");
        assert_display!(stl.italic(), "\x1b[3m");
        assert_display!(stl.underline(), "\x1b[4m");
        assert_display!(stl.curly_underline(), "\x1b[4:3m");
        assert_display!(stl.dotted_underline(), "\x1b[4:4m");
        assert_display!(stl.dashed_underline(), "\x1b[4:5m");
        assert_display!(stl.blink(), "\x1b[5m");
        assert_display!(stl.reverse(), "\x1b[7m");
        assert_display!(stl.conceal(), "\x1b[8m");
        assert_display!(stl.strikethrough(), "\x1b[9m");
        assert_display!(stl.double_underline(), "\x1b[21m");
        assert_display!(stl.overline(), "\x1b[53m");
    }

    #[test]
    fn colors_display() {
        let stl = Style::new();

        assert_display!(stl.fg(BasicColor::Red), "\x1b[31m");
        assert_display!(stl.bg(BasicColor::Red), "\x1b[41m");
    }

    #[test]
    fn combined_display() {
        let stl = Style::new()
            .bold()
            .fg(BasicColor::Red)
            .underline()
            .bg(BasicColor::Green);
        assert_display!(stl, "\x1b[1;4;31;42m");
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
