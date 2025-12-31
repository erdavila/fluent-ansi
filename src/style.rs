use core::fmt::{Display, Formatter, Result, Write};

use crate::{
    ColorTarget, Effect, Reset, StyleSet, ToStyleSet, UnderlineStyle,
    colors::{Color, WriteColorCodes as _},
    impl_macros::applied_to::impl_applied_to,
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

    impl_applied_to!();

    /// Converts the type into a [`Style`].
    #[must_use]
    pub fn to_style(self) -> Style {
        self
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
