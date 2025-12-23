use core::fmt::{Display, Formatter, Result};

use enum_iterator::Sequence;

use crate::{AppliedTo, Style, StyleAttribute, StyleElement, StyleSet, ToStyle, ToStyleSet};

/// An enumeration of all supported text styling effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
pub enum Effect {
    /// Bold styling.
    Bold,
    /// Faint styling.
    Faint,
    /// Italic styling.
    Italic,
    /// Underline styling.
    Underline,
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
    pub(crate) const fn get_code(self) -> u8 {
        match self {
            Effect::Bold => 1,
            Effect::Faint => 2,
            Effect::Italic => 3,
            Effect::Underline => 4,
            Effect::Blink => 5,
            Effect::Reverse => 7,
            Effect::Conceal => 8,
            Effect::Strikethrough => 9,
            Effect::DoubleUnderline => 21,
            Effect::Overline => 53,
        }
    }

    #[must_use]
    const fn bit_mask(self) -> u16 {
        let bit_index = self as u16;
        1 << bit_index
    }
}

impl StyleElement for Effect {
    fn add_to_style(self, style: Style) -> Style {
        style.set_effect(self, true)
    }
}

impl StyleAttribute for Effect {
    type Value = bool;

    fn set_in_style(self, style: Style, value: Self::Value) -> Style {
        let effects = if value {
            style.effects | self.bit_mask()
        } else {
            style.effects & !self.bit_mask()
        };
        Style { effects, ..style }
    }

    fn get_from_style(self, style: &Style) -> Self::Value {
        style.effects & self.bit_mask() != 0
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

#[cfg(test)]
mod tests {
    use crate::{
        ToStyleSet as _, assert_display,
        color::{BasicColor, ColorKind as _},
    };

    use super::*;

    #[test]
    fn add_effect() {
        let effect = Effect::Bold;

        assert_eq!(effect.bold(), Style::new().bold());
        assert_eq!(effect.italic(), Style::new().bold().italic());
        assert_eq!(effect.effect(Effect::Italic), Style::new().bold().italic());
        assert_eq!(effect.add(Effect::Italic), Style::new().bold().italic());
    }

    #[test]
    fn add_color() {
        let effect = Effect::Bold;

        assert_eq!(
            effect.fg(BasicColor::Green),
            Style::new().bold().fg(BasicColor::Green)
        );
        assert_eq!(
            effect.color(BasicColor::Green.in_bg()),
            Style::new().bold().bg(BasicColor::Green)
        );
        assert_eq!(
            effect.add(BasicColor::Green.in_bg()),
            Style::new().bold().bg(BasicColor::Green)
        );
    }

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
    fn to_style_set() {
        assert_eq!(Effect::Bold.to_style_set(), Style::new().bold());
    }

    #[test]
    fn display() {
        assert_display!(Effect::Bold, "\x1b[1m");
        assert_display!(Effect::Faint, "\x1b[2m");
        assert_display!(Effect::Italic, "\x1b[3m");
        assert_display!(Effect::Underline, "\x1b[4m");
        assert_display!(Effect::Blink, "\x1b[5m");
        assert_display!(Effect::Reverse, "\x1b[7m");
        assert_display!(Effect::Conceal, "\x1b[8m");
        assert_display!(Effect::Strikethrough, "\x1b[9m");
        assert_display!(Effect::DoubleUnderline, "\x1b[21m");
        assert_display!(Effect::Overline, "\x1b[53m");
    }
}
