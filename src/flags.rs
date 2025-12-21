use core::fmt::{Display, Formatter, Result};

use enum_iterator::Sequence;

use crate::{AppliedTo, Style, StyleAttribute, StyleElement, StyleSet, ToStyle, ToStyleSet};

/// An enumeration of all supported text styling flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
pub enum Flag {
    /// Bold styling.
    Bold,
    /// Faint styling.
    Faint,
    /// Italic styling.
    Italic,
    /// Underline styling.
    Underline,
    /// Slow blink styling.
    SlowBlink,
    /// Rapid blink styling.
    RapidBlink,
    /// Reverse video styling.
    Reverse,
    /// Conceal (hidden) styling.
    Conceal,
    /// Crossed-out (strikethrough) styling.
    CrossedOut,
    /// Double underline styling.
    DoubleUnderline,
    /// Overline styling.
    Overline,
}

impl Flag {
    #[must_use]
    pub(crate) const fn get_code(self) -> u8 {
        match self {
            Flag::Bold => 1,
            Flag::Faint => 2,
            Flag::Italic => 3,
            Flag::Underline => 4,
            Flag::SlowBlink => 5,
            Flag::RapidBlink => 6,
            Flag::Reverse => 7,
            Flag::Conceal => 8,
            Flag::CrossedOut => 9,
            Flag::DoubleUnderline => 21,
            Flag::Overline => 53,
        }
    }

    #[must_use]
    const fn bit_mask(self) -> u16 {
        let bit_index = self as u16;
        1 << bit_index
    }
}

impl StyleElement for Flag {
    fn add_to_style(self, style: Style) -> Style {
        style.set_flag(self, true)
    }
}

impl StyleAttribute for Flag {
    type Value = bool;

    fn set_in_style(self, style: Style, value: Self::Value) -> Style {
        let flags = if value {
            style.flags | self.bit_mask()
        } else {
            style.flags & !self.bit_mask()
        };
        Style { flags, ..style }
    }

    fn get_from_style(self, style: &Style) -> Self::Value {
        style.flags & self.bit_mask() != 0
    }
}

impl ToStyleSet for Flag {
    type StyleSet = Style;

    fn to_style_set(self) -> Self::StyleSet {
        self.to_style()
    }
}

impl ToStyle for Flag {
    fn to_style(self) -> Style {
        self.into()
    }
}

impl AppliedTo for Flag {}

impl Display for Flag {
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
    fn add_flag() {
        let flag = Flag::Bold;

        assert_eq!(flag.bold(), Style::new().bold());
        assert_eq!(flag.italic(), Style::new().bold().italic());
        assert_eq!(flag.flag(Flag::Italic), Style::new().bold().italic());
        assert_eq!(flag.add(Flag::Italic), Style::new().bold().italic());
    }

    #[test]
    fn add_color() {
        let flag = Flag::Bold;

        assert_eq!(
            flag.fg(BasicColor::Green),
            Style::new().bold().fg(BasicColor::Green)
        );
        assert_eq!(
            flag.color(BasicColor::Green.in_bg()),
            Style::new().bold().bg(BasicColor::Green)
        );
        assert_eq!(
            flag.add(BasicColor::Green.in_bg()),
            Style::new().bold().bg(BasicColor::Green)
        );
    }

    #[test]
    fn applied_to() {
        let stld = Flag::Bold.applied_to("CONTENT");

        assert_eq!(stld.get_content(), &"CONTENT");
        assert_eq!(stld.get_style(), Style::new().bold());
    }

    #[test]
    fn to_style() {
        assert_eq!(Flag::Bold.to_style(), Style::new().bold());
    }

    #[test]
    fn to_style_set() {
        assert_eq!(Flag::Bold.to_style_set(), Style::new().bold());
    }

    #[test]
    fn display() {
        assert_display!(Flag::Bold, "\x1b[1m");
        assert_display!(Flag::Faint, "\x1b[2m");
        assert_display!(Flag::Italic, "\x1b[3m");
        assert_display!(Flag::Underline, "\x1b[4m");
        assert_display!(Flag::SlowBlink, "\x1b[5m");
        assert_display!(Flag::RapidBlink, "\x1b[6m");
        assert_display!(Flag::Reverse, "\x1b[7m");
        assert_display!(Flag::Conceal, "\x1b[8m");
        assert_display!(Flag::CrossedOut, "\x1b[9m");
        assert_display!(Flag::DoubleUnderline, "\x1b[21m");
        assert_display!(Flag::Overline, "\x1b[53m");
    }
}
