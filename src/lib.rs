#![no_std]
#![warn(clippy::pedantic)]

use core::fmt::{Display, Formatter, Result, Write};

fn write_escape_sequence(f: &mut impl Write, codes: impl Display) -> Result {
    write!(f, "\x1b[{codes}m")
}

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
        write!(f, "{}", self.to_format())
    }
}

mod private {
    use crate::Format;

    pub trait WithFormat: Sized {
        fn modify_format(self, m: impl Fn(&mut Format)) -> Self {
            let mut format = self.get_format();
            m(&mut format);
            self.with_format(format)
        }

        fn get_format(&self) -> Format;

        fn with_format(self, format: Format) -> Self;
    }
}
use private::WithFormat as PrivateWithFormat;

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
    use arrayvec::ArrayString;

    use super::*;

    macro_rules! assert_display {
        ($display:expr, $expected:literal) => {{
            let mut vec = ArrayString::<20>::new();

            write!(&mut vec, "{}", $display).unwrap();

            assert_eq!(vec.as_str(), $expected);
        }};
    }

    #[test]
    fn color_to_format() {
        assert_eq!(Color::Red.to_format(), Format::new().color(Color::Red));
    }

    #[test]
    fn color_display() {
        assert_display!(Color::Black, "\x1b[30m");
        assert_display!(Color::Red, "\x1b[31m");
        assert_display!(Color::Green, "\x1b[32m");
        assert_display!(Color::Yellow, "\x1b[33m");
        assert_display!(Color::Blue, "\x1b[34m");
        assert_display!(Color::Magenta, "\x1b[35m");
        assert_display!(Color::Cyan, "\x1b[36m");
        assert_display!(Color::White, "\x1b[37m");
    }

    #[test]
    fn format_bold() {
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
    fn format_color() {
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
    fn format_combined() {
        let fmt = Format::new().bold().color(Color::Red);
        assert_display!(fmt, "\x1b[1;31m");
    }

    #[test]
    fn format_default() {
        assert_display!(Format::default(), "\x1b[0m");
    }

    #[test]
    fn format_from_color() {
        assert_eq!(Format::from(Color::Red), Format::new().color(Color::Red));
    }

    #[test]
    fn formatted_content() {
        let fmtd = Formatted::new("CONTENT");

        assert_eq!(fmtd.get_content(), &"CONTENT");
        assert_display!(fmtd, "CONTENT");
    }

    #[test]
    fn formatted_bold() {
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
    fn formatted_color() {
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
    fn formatted_combined() {
        let fmtd = Formatted::new("CONTENT").bold().color(Color::Red);
        assert_eq!(fmtd.get_format(), Format::new().bold().color(Color::Red));
        assert_display!(fmtd, "\x1b[1;31mCONTENT\x1b[0m");
    }
}
