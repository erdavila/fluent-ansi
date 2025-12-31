use core::fmt::{Display, Formatter, Result};

use enum_iterator::Sequence;

use crate::{
    CodeWriter, Style, StyleAttribute, StyleElement, StyleSet, ToStyleSet,
    impl_macros::applied_to::impl_applied_to,
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
    Underline,
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
    impl_applied_to!();

    /// Converts the effect into a [`Style`].
    #[must_use]
    pub fn to_style(self) -> Style {
        Style::new().effect(self)
    }

    #[must_use]
    pub(crate) fn all() -> AllEffects {
        enum_iterator::all()
    }

    pub(crate) fn write_codes(self, code_writer: &mut CodeWriter) -> Result {
        let codes = match self {
            Effect::Bold => "1",
            Effect::Faint => "2",
            Effect::Italic => "3",
            Effect::Underline => "4",
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

impl StyleElement for Effect {
    fn add_to<S: StyleSet>(self, style_set: S) -> S {
        style_set.set_effect(self, true)
    }
}

impl StyleAttribute for Effect {
    type Value = bool;

    fn set_in<S: StyleSet>(self, style_set: S, value: Self::Value) -> S {
        style_set.set_effect(self, value)
    }

    fn get_from<S: StyleSet>(self, style_set: &S) -> Self::Value {
        style_set.get_effect(self)
    }
}

impl ToStyleSet for Effect {
    type StyleSet = Style;

    fn to_style_set(self) -> Self::StyleSet {
        self.to_style()
    }
}

impl Display for Effect {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.to_style().fmt(f)
    }
}

impl From<UnderlineStyle> for Effect {
    fn from(value: UnderlineStyle) -> Self {
        value.to_effect()
    }
}
