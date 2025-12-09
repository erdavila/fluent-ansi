use core::fmt::{Display, Formatter, Result};

use crate::{Format, private::PrivateWithFormat};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Formatted<C: Display> {
    content: C,
    format: Format,
}
impl<C: Display> Formatted<C> {
    #[must_use]
    pub fn new(content: C) -> Self {
        Self {
            content,
            format: Format::default(),
        }
    }

    #[must_use]
    pub fn get_content(&self) -> &C {
        &self.content
    }

    #[must_use]
    pub fn get_format(&self) -> Format {
        <Self as PrivateWithFormat>::get_format(self)
    }
}
impl<C: Display> PrivateWithFormat for Formatted<C> {
    fn get_format(&self) -> Format {
        self.format
    }

    fn with_format(self, format: Format) -> Self {
        Self { format, ..self }
    }
}
impl<C: Display> Display for Formatted<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.format == Format::default() {
            write!(f, "{}", self.content)
        } else {
            let start = self.format;
            let end = Format::default();
            write!(f, "{start}{}{end}", self.content)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Color, WithFormat as _, assert_display};

    use super::*;

    #[test]
    fn content() {
        let fmtd = Formatted::new("CONTENT");

        assert_eq!(fmtd.get_content(), &"CONTENT");
        assert_display!(fmtd, "CONTENT");
    }

    #[test]
    fn bold() {
        let fmtd = Formatted::new("CONTENT");
        assert_eq!(fmtd.is_bold(), false);

        let fmtd = fmtd.bold();
        assert_display!(fmtd, "\x1b[1mCONTENT\x1b[0m");
        assert_eq!(fmtd.is_bold(), true);

        let fmtd = fmtd.no_bold();
        assert_eq!(fmtd.is_bold(), false);

        let fmtd = fmtd.set_bold(true);
        assert_eq!(fmtd.is_bold(), true);

        let fmtd = fmtd.set_bold(false);
        assert_eq!(fmtd.is_bold(), false);
    }

    #[test]
    fn color() {
        let fmtd = Formatted::new("CONTENT");
        assert_eq!(fmtd.get_color(), None);

        let fmtd = fmtd.color(Color::Red);
        assert_display!(fmtd, "\x1b[31mCONTENT\x1b[0m");
        assert_eq!(fmtd.get_color(), Some(Color::Red));

        let fmtd = fmtd.no_color();
        assert_eq!(fmtd.get_color(), None);

        let fmtd = fmtd.with_color(Some(Color::Red));
        assert_eq!(fmtd.get_color(), Some(Color::Red));

        let fmtd = fmtd.with_color(None);
        assert_eq!(fmtd.get_color(), None);
    }

    #[test]
    fn combined() {
        let fmtd = Formatted::new("CONTENT").bold().color(Color::Red);
        assert_eq!(fmtd.get_format(), Format::new().bold().color(Color::Red));
        assert_display!(fmtd, "\x1b[1;31mCONTENT\x1b[0m");
    }
}
