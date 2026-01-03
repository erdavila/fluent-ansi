use core::fmt::{Display, Formatter, Result};

use enum_iterator::Sequence;

use crate::{
    CodeWriter, Style, impl_macros::fluent::impl_fluent_type, impl_style_atribute_for,
    impl_style_element_for,
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

impl_fluent_type!(Effect {
    args: [self];
    to_style: { Style::new().effect(self) }
});

impl Effect {
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

impl_style_element_for! { Effect {
    args: [self, composed_styling];
    add_to: {
        composed_styling.set_effect(self, true)
    }
}}

impl_style_atribute_for! { Effect {
    type Value = bool;
    args: [self, composed_styling, value];

    set_in: {
        composed_styling.set_effect(self, value)
    }

    get_from: {
        composed_styling.get_effect(self)
    }
}}

impl Display for Effect {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.to_style().fmt(f)
    }
}
