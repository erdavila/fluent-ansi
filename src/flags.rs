use core::fmt::Display;

use enum_iterator::Sequence;

use crate::{Format, Formatted};

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
}

impl Display for Flag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.to_format().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Add as _, assert_display};

    use super::*;

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
