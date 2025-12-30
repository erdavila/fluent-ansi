use core::fmt::{Display, Formatter, Result, Write};

use crate::{
    AppliedTo, ColorTarget, Effect, Reset, StyleSet, Styled, TargetedColor, ToStyle, ToStyleSet,
    UnderlineStyle,
    colors::{Color, WriteColorCodes as _},
    style::encoded_effects::EncodedEffects,
};

pub use encoded_effects::*;

mod encoded_effects;

/// A structure representing text styling with effects and colors.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Style {
    encoded_effects: EncodedEffects,
    fg: Option<Color>,
    bg: Option<Color>,
    underline_color: Option<Color>,
}

impl Style {
    /// Creates a new, empty `Style` value.
    #[must_use]
    pub const fn new() -> Self {
        Style {
            encoded_effects: EncodedEffects::new(),
            fg: None,
            bg: None,
            underline_color: None,
        }
    }
}

impl ToStyleSet for Style {
    type StyleSet = Self;

    fn to_style_set(self) -> Self::StyleSet {
        self
    }
}

impl StyleSet for Style {
    fn set_effect(self, effect: impl Into<Effect>, value: bool) -> Self {
        let effect = effect.into();
        let encoded_effects = self.encoded_effects.set(effect, value);
        Self {
            encoded_effects,
            ..self
        }
    }

    fn get_effect(&self, effect: impl Into<Effect>) -> bool {
        let effect = effect.into();
        self.encoded_effects.get(effect)
    }

    fn get_effects(&self) -> GetEffects {
        self.encoded_effects.get_effects()
    }

    fn set_underline_style(self, underline_style: Option<UnderlineStyle>) -> Self {
        let encoded_effects = self.encoded_effects.set_underline(underline_style);
        Self {
            encoded_effects,
            ..self
        }
    }

    fn get_underline_style(&self) -> Option<UnderlineStyle> {
        UnderlineStyle::all().find(|&underline_style| self.get_effect(underline_style))
    }

    fn set_color(self, target: ColorTarget, color: Option<impl Into<Color>>) -> Self {
        let color = color.map(Into::into);
        match target {
            ColorTarget::Foreground => Self { fg: color, ..self },
            ColorTarget::Background => Self { bg: color, ..self },
            ColorTarget::Underline => Self {
                underline_color: color,
                ..self
            },
        }
    }

    fn get_color(&self, target: ColorTarget) -> Option<Color> {
        match target {
            ColorTarget::Foreground => self.fg,
            ColorTarget::Background => self.bg,
            ColorTarget::Underline => self.underline_color,
        }
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
                        color.write_color_codes(ColorTarget::Foreground, &mut code_writer)?;
                    }
                    if let Some(color) = self.0.bg {
                        color.write_color_codes(ColorTarget::Background, &mut code_writer)?;
                    }
                    if let Some(color) = self.0.underline_color {
                        color.write_color_codes(ColorTarget::Underline, &mut code_writer)?;
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
        effect.to_style()
    }
}

impl From<UnderlineStyle> for Style {
    fn from(underline_style: UnderlineStyle) -> Self {
        underline_style.to_style()
    }
}

impl From<TargetedColor> for Style {
    fn from(targeted_color: TargetedColor) -> Self {
        targeted_color.to_style()
    }
}

impl<C: Into<Color>> From<C> for Style {
    fn from(color: C) -> Self {
        color.to_style()
    }
}

impl From<Reset> for Style {
    fn from(reset: Reset) -> Self {
        reset.to_style()
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
        colors::{BasicColor, IndexedColor, RGBColor, SimpleColor},
        style_set::tests::test_style_set_methods,
        tests::assert_display,
        to_style_set::tests::test_to_style_set_methods,
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
    fn from_targeted_color() {
        assert_eq!(
            Style::from(BasicColor::Red.for_fg()),
            Style::new().color(BasicColor::Red.for_fg())
        );
    }

    #[test]
    fn from_color() {
        assert_eq!(
            Style::from(BasicColor::Red),
            Style::new().color(BasicColor::Red.for_fg())
        );
        assert_eq!(
            Style::from(SimpleColor::new(BasicColor::Red)),
            Style::new().color(SimpleColor::new(BasicColor::Red).for_fg())
        );
        assert_eq!(
            Style::from(IndexedColor(42)),
            Style::new().color(IndexedColor(42).for_fg())
        );
        assert_eq!(
            Style::from(RGBColor::new(0, 128, 255)),
            Style::new().color(RGBColor::new(0, 128, 255).for_fg())
        );
    }

    #[test]
    fn from_reset() {
        assert_eq!(Style::from(Reset), Style::new());
    }
}
