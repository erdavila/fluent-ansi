use core::fmt::{Display, Formatter, Result};

use enum_iterator::Sequence;

use crate::{
    CodeWriter, Style,
    macros::impl_add_for_additive_type,
    traits::{Composed, StylingAttribute, StylingElement, ToStyle as _},
};
pub use underline::*;

mod underline;

pub(crate) type AllEffects = enum_iterator::All<Effect>;

/// An enumeration of all supported text styling effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
pub enum Effect {
    /// Bold styling.
    Bold,
    /// Faint styling.
    Faint,
    /// Italic styling.
    Italic,
    /// Solid underline styling.
    SolidUnderline,
    /// Curly underline styling.
    CurlyUnderline,
    /// Dotted underline styling.
    DottedUnderline,
    /// Dashed underline styling.
    DashedUnderline,
    /// Blink styling.
    Blink,
    /// Reverse video styling.
    Reverse,
    /// Conceal (hidden) styling.
    Conceal,
    /// Strikethrough styling.
    Strikethrough,
    /// Double underline styling.
    DoubleUnderline,
    /// Overline styling.
    Overline,
}

impl Effect {
    /// An alias for [`Effect::SolidUnderline`].
    pub const UNDERLINE: Effect = Effect::SolidUnderline;

    #[must_use]
    pub(crate) fn all() -> AllEffects {
        enum_iterator::all()
    }

    pub(crate) fn write_codes(self, code_writer: &mut CodeWriter) -> Result {
        let codes = match self {
            Effect::Bold => "1",
            Effect::Faint => "2",
            Effect::Italic => "3",
            Effect::SolidUnderline => "4",
            Effect::CurlyUnderline => "4:3",
            Effect::DottedUnderline => "4:4",
            Effect::DashedUnderline => "4:5",
            Effect::Blink => "5",
            Effect::Reverse => "7",
            Effect::Conceal => "8",
            Effect::Strikethrough => "9",
            Effect::DoubleUnderline => "21",
            Effect::Overline => "53",
        };
        code_writer.write_code(codes)
    }
}

impl StylingElement for Effect {
    fn add_to<C: Composed>(self, composed: C) -> C {
        composed.set_effect(self, true)
    }
}

impl StylingAttribute for Effect {
    type Value = bool;

    fn set_in<C: Composed>(self, composed: C, value: Self::Value) -> C {
        composed.set_effect(self, value)
    }

    fn get_from<C: Composed>(self, composed: &C) -> Self::Value {
        composed.get_effect(self)
    }
}

impl Display for Effect {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.to_style().fmt(f)
    }
}

impl From<UnderlineEffect> for Effect {
    fn from(underline_effect: UnderlineEffect) -> Self {
        match underline_effect {
            UnderlineEffect::Solid => Effect::SolidUnderline,
            UnderlineEffect::Curly => Effect::CurlyUnderline,
            UnderlineEffect::Dotted => Effect::DottedUnderline,
            UnderlineEffect::Dashed => Effect::DashedUnderline,
            UnderlineEffect::Double => Effect::DoubleUnderline,
        }
    }
}

impl_add_for_additive_type!(for Effect, Output = Style);
