use core::fmt::{Display, Formatter, Result, Write};

use crate::{
    ColorTarget, Effect, Reset, UnderlineEffect,
    colors::{Color, WriteColorCodes as _},
    impl_macros::{
        additive_styling::impl_additive_styling_type,
        composed_styling::impl_composed_styling_methods,
    },
    style::encoded_effects::EncodedEffects,
};

pub use encoded_effects::*;

mod encoded_effects;

/// A structure representing text styling with effects and colors.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Style {
    encoded_effects: EncodedEffects,
    foreground_color: Option<Color>,
    background_color: Option<Color>,
    underline_color: Option<Color>,
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
        }
    }

    impl_composed_styling_methods! {
        args: [self, effect, underline_effect, target, color, value];
        example_variable: r"style";

        set_effect: {
            let effect = effect.into();
            let encoded_effects = self.encoded_effects.set(effect, value);
            Self {
                encoded_effects,
                ..self
            }
        }

        get_effect: {
            let effect = effect.into();
            self.encoded_effects.get(effect)
        }

        get_effects: {
            self.encoded_effects.get_effects()
        }

        set_underline_effect: {
            let encoded_effects = self.encoded_effects.set_underline(underline_effect);
            Self {
                encoded_effects,
                ..self
            }
        }

        get_underline_effect: {
            UnderlineEffect::all().find(|&underline_effect| self.get_effect(underline_effect))
        }

        set_color: {
            let color = color.map(Into::into);
            match target {
                ColorTarget::Foreground => Self { foreground_color: color, ..self },
                ColorTarget::Background => Self { background_color: color, ..self },
                ColorTarget::Underline => Self {
                    underline_color: color,
                    ..self
                },
            }
        }

        get_color: {
            match target {
                ColorTarget::Foreground => self.foreground_color,
                ColorTarget::Background => self.background_color,
                ColorTarget::Underline => self.underline_color,
            }
        }
    }
}

impl_additive_styling_type!(Style {
    args: [self];
    to_style: SELF
});

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
