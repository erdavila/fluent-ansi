use core::fmt::{Display, Formatter, Result};

use enum_iterator::Sequence;

use crate::{
    AppliedTo, CodeWriter, Style, StyleAttribute, StyleElement, StyleSet, ToStyle, ToStyleSet,
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

impl ToStyle for Effect {
    fn to_style(self) -> Style {
        self.into()
    }
}

impl AppliedTo for Effect {}

impl Display for Effect {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.to_style().fmt(f)
    }
}

impl From<UnderlineStyle> for Effect {
    fn from(value: UnderlineStyle) -> Self {
        match value {
            UnderlineStyle::Solid => Effect::Underline,
            UnderlineStyle::Curly => Effect::CurlyUnderline,
            UnderlineStyle::Dotted => Effect::DottedUnderline,
            UnderlineStyle::Dashed => Effect::DashedUnderline,
            UnderlineStyle::Double => Effect::DoubleUnderline,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ToStyleSet as _, tests::assert_display, to_style_set::tests::test_to_style_set_methods,
    };

    use super::*;

    test_to_style_set_methods!(bold; Effect::Bold, Style::new().bold());
    test_to_style_set_methods!(italic; Effect::Italic, Style::new().italic());

    #[test]
    fn applied_to() {
        let stld = Effect::Bold.applied_to("CONTENT");

        assert_eq!(stld.get_content(), &"CONTENT");
        assert_eq!(stld.get_style(), Style::new().bold());
    }

    #[test]
    fn to_style() {
        assert_eq!(Effect::Bold.to_style(), Style::new().bold());
    }

    #[test]
    fn display() {
        assert_display!(Effect::Bold, "\x1b[1m");
        assert_display!(Effect::Faint, "\x1b[2m");
        assert_display!(Effect::Italic, "\x1b[3m");
        assert_display!(Effect::Underline, "\x1b[4m");
        assert_display!(Effect::CurlyUnderline, "\x1b[4:3m");
        assert_display!(Effect::DottedUnderline, "\x1b[4:4m");
        assert_display!(Effect::DashedUnderline, "\x1b[4:5m");
        assert_display!(Effect::Blink, "\x1b[5m");
        assert_display!(Effect::Reverse, "\x1b[7m");
        assert_display!(Effect::Conceal, "\x1b[8m");
        assert_display!(Effect::Strikethrough, "\x1b[9m");
        assert_display!(Effect::DoubleUnderline, "\x1b[21m");
        assert_display!(Effect::Overline, "\x1b[53m");
    }
}
