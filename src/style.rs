use core::fmt::{Display, Formatter, Result, Write};

use crate::{
    ColorTarget, Effect, UnderlineEffect,
    colors::{Color, WriteColorCodes as _},
    impl_macros::additive_styling::impl_additive_styling_type,
    style::encoded_effects::EncodedEffects,
    traits::{Composed, StylingElement},
};

pub use encoded_effects::*;

mod encoded_effects;

/// A structure representing text styling with effects and colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Style {
    encoded_effects: EncodedEffects,
    foreground_color: Option<Color>,
    background_color: Option<Color>,
    underline_color: Option<Color>,
    enabled: bool,
}

impl Style {
    /// Creates a new, empty `Style` value.
    #[must_use]
    pub const fn new() -> Self {
        Style {
            encoded_effects: EncodedEffects::new(),
            foreground_color: None,
            background_color: None,
            underline_color: None,
            enabled: true,
        }
    }
}

impl Composed for Style {
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

    fn set_underline_effect(self, underline_effect: Option<UnderlineEffect>) -> Self {
        let encoded_effects = self.encoded_effects.set_underline(underline_effect);
        Self {
            encoded_effects,
            ..self
        }
    }

    fn get_underline_effect(&self) -> Option<UnderlineEffect> {
        UnderlineEffect::all().find(|&underline_effect| self.get_effect(underline_effect))
    }

    fn set_color(self, target: ColorTarget, color: Option<impl Into<Color>>) -> Self {
        let color = color.map(Into::into);
        match target {
            ColorTarget::Foreground => Self {
                foreground_color: color,
                ..self
            },
            ColorTarget::Background => Self {
                background_color: color,
                ..self
            },
            ColorTarget::Underline => Self {
                underline_color: color,
                ..self
            },
        }
    }

    fn get_color(&self, target: ColorTarget) -> Option<Color> {
        match target {
            ColorTarget::Foreground => self.foreground_color,
            ColorTarget::Background => self.background_color,
            ColorTarget::Underline => self.underline_color,
        }
    }

    fn merge_style(self, other: Style) -> Self {
        Self {
            encoded_effects: self.encoded_effects.merge(other.encoded_effects),
            foreground_color: other.foreground_color.or(self.foreground_color),
            background_color: other.background_color.or(self.background_color),
            underline_color: other.underline_color.or(self.underline_color),
            enabled: other.enabled,
        }
    }

    fn set_enabled(self, enabled: bool) -> Self {
        Self { enabled, ..self }
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl_additive_styling_type!(Style {
    args: [self];
    to_style: { self }
});

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if !self.enabled {
            Ok(())
        } else if *self == Style::new() {
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
                    if let Some(color) = self.0.foreground_color {
                        color.write_color_codes(ColorTarget::Foreground, &mut code_writer)?;
                    }
                    if let Some(color) = self.0.background_color {
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

impl<T> From<T> for Style
where
    T: StylingElement,
{
    fn from(element: T) -> Self {
        element.add_to(Style::new())
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
