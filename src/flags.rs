use core::fmt::Display;

use enum_iterator::Sequence;

use crate::{Format, FormatElement, FormatSet, Formatted, Position, ToFormatSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
pub enum Flag {
    Bold,
    Faint,
    Italic,
    Underline,
    SlowBlink,
    RapidBlink,
    Reverse,
    Conceal,
    CrossedOut,
    DoubleUnderline,
    Overline,
}

impl Flag {
    #[must_use]
    pub fn applied_to<C: Display>(self, content: C) -> Formatted<C> {
        self.to_format().applied_to(content)
    }

    #[must_use]
    pub fn to_format(self) -> Format {
        self.into()
    }

    #[must_use]
    pub(crate) fn get_code(self) -> u8 {
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
    fn bit_mask(self) -> u16 {
        let bit_index = self as u16;
        1 << bit_index
    }
}

impl FormatElement for Flag {
    fn add_to_format(self, format: Format) -> Format {
        format.set_flag(self, true)
    }
}

impl Position for Flag {
    type Value = bool;

    fn set_in_format(self, format: Format, value: Self::Value) -> Format {
        let flags = if value {
            format.flags | self.bit_mask()
        } else {
            format.flags & !self.bit_mask()
        };
        Format { flags, ..format }
    }

    fn get_from_format(self, format: &Format) -> Self::Value {
        format.flags & self.bit_mask() != 0
    }
}

impl ToFormatSet for Flag {
    type FormatSet = Format;

    fn to_format_set(self) -> Self::FormatSet {
        Format::new().flag(self)
    }
}

impl Display for Flag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.to_format().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Color, ToFormatSet as _, assert_display};

    use super::*;

    #[test]
    fn add_flag() {
        let flag = Flag::Bold;

        assert_eq!(flag.bold(), Format::new().bold());
        assert_eq!(flag.italic(), Format::new().bold().italic());
        assert_eq!(flag.flag(Flag::Italic), Format::new().bold().italic());
        assert_eq!(flag.add(Flag::Italic), Format::new().bold().italic());
    }

    #[test]
    fn add_color() {
        let flag = Flag::Bold;

        assert_eq!(flag.fg(Color::Green), Format::new().bold().fg(Color::Green));
        assert_eq!(
            flag.color(Color::Green.bg()),
            Format::new().bold().bg(Color::Green)
        );
        assert_eq!(
            flag.add(Color::Green.bg()),
            Format::new().bold().bg(Color::Green)
        );
    }

    #[test]
    fn applied_to() {
        let fmtd = Flag::Bold.applied_to("CONTENT");

        assert_eq!(fmtd.get_content(), &"CONTENT");
        assert_eq!(fmtd.get_format(), Format::new().bold());
    }

    #[test]
    fn to_format() {
        assert_eq!(Flag::Bold.to_format(), Format::new().bold());
    }

    #[test]
    fn to_format_set() {
        assert_eq!(Flag::Bold.to_format_set(), Format::new().bold());
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
