use core::fmt::{Display, Formatter, Result, Write};

use crate::{Color, private::PrivateWithFormat};

pub trait WithFormat: PrivateWithFormat {
    #[must_use]
    fn bold(self) -> Self {
        self.set_bold(true)
    }

    #[must_use]
    fn no_bold(self) -> Self {
        self.set_bold(false)
    }

    #[must_use]
    fn set_bold(self, bold: bool) -> Self {
        self.modify_format(|fmt| fmt.bold = bold)
    }

    #[must_use]
    fn is_bold(&self) -> bool {
        self.get_format().bold
    }

    #[must_use]
    fn color(self, color: Color) -> Self {
        self.with_color(Some(color))
    }

    #[must_use]
    fn no_color(self) -> Self {
        self.with_color(None)
    }

    #[must_use]
    fn with_color(self, color: Option<Color>) -> Self {
        self.modify_format(|fmt| fmt.color = color)
    }

    #[must_use]
    fn get_color(self) -> Option<Color> {
        self.get_format().color
    }
}
// Automatically implement WithFormat for every type that implements PrivateWithFormat
impl<T: PrivateWithFormat> WithFormat for T {}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Format {
    bold: bool,
    color: Option<Color>,
}
impl Format {
    #[must_use]
    pub fn new() -> Self {
        Format::default()
    }
}
impl PrivateWithFormat for Format {
    fn get_format(&self) -> Format {
        *self
    }

    fn with_format(self, format: Format) -> Self {
        format
    }
}
impl Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if *self == Format::new() {
            write_escape_sequence(f, 0)
        } else {
            struct Codes(Format);
            impl Display for Codes {
                fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                    if self.0.bold {
                        f.write_char('1')?;
                    }
                    if self.0.bold && self.0.color.is_some() {
                        f.write_char(';')?;
                    }
                    if let Some(color) = self.0.color {
                        write!(f, "{}", 30 + color as u8)?;
                    }
                    Ok(())
                }
            }
            write_escape_sequence(f, Codes(*self))
        }
    }
}
impl From<Color> for Format {
    fn from(color: Color) -> Self {
        Format::new().color(color)
    }
}

fn write_escape_sequence(f: &mut impl Write, codes: impl Display) -> Result {
    write!(f, "\x1b[{codes}m")
}

#[cfg(test)]
mod tests {
    use crate::assert_display;

    use super::*;

    #[test]
    fn bold() {
        let fmt = Format::new();
        assert!(!fmt.is_bold());

        let fmt = fmt.bold();
        assert_display!(fmt, "\x1b[1m");
        assert!(fmt.is_bold());

        let fmt = fmt.no_bold();
        assert!(!fmt.is_bold());
        assert_eq!(fmt, Format::default());

        let fmt = fmt.set_bold(true);
        assert!(fmt.is_bold());

        let fmt = fmt.set_bold(false);
        assert!(!fmt.is_bold());
    }

    #[test]
    fn color() {
        let fmt = Format::new();
        assert_eq!(fmt.get_color(), None);

        let fmt = fmt.color(Color::Red);
        assert_display!(fmt, "\x1b[31m");
        assert_eq!(fmt.get_color(), Some(Color::Red));

        let fmt = fmt.no_color();
        assert_eq!(fmt.get_color(), None);
        assert_eq!(fmt, Format::default());

        let fmt = fmt.with_color(Some(Color::Red));
        assert_eq!(fmt.get_color(), Some(Color::Red));

        let fmt = fmt.with_color(None);
        assert_eq!(fmt.get_color(), None);
    }

    #[test]
    fn combined() {
        let fmt = Format::new().bold().color(Color::Red);
        assert_display!(fmt, "\x1b[1;31m");
    }

    #[test]
    fn default() {
        assert_display!(Format::default(), "\x1b[0m");
    }

    #[test]
    fn from_color() {
        assert_eq!(Format::from(Color::Red), Format::new().color(Color::Red));
    }
}
