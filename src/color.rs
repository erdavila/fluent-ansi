use core::fmt::{Display, Formatter, Result};

use crate::Format;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
    #[must_use]
    pub fn to_format(self) -> Format {
        self.into()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.to_format().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{WithFormat as _, assert_display};

    use super::*;

    #[test]
    fn to_format() {
        assert_eq!(Color::Red.to_format(), Format::new().color(Color::Red));
    }

    #[test]
    fn display() {
        assert_display!(Color::Black, "\x1b[30m");
        assert_display!(Color::Red, "\x1b[31m");
        assert_display!(Color::Green, "\x1b[32m");
        assert_display!(Color::Yellow, "\x1b[33m");
        assert_display!(Color::Blue, "\x1b[34m");
        assert_display!(Color::Magenta, "\x1b[35m");
        assert_display!(Color::Cyan, "\x1b[36m");
        assert_display!(Color::White, "\x1b[37m");
    }
}
