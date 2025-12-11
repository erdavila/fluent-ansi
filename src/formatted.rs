use core::fmt::{Display, Formatter, Result};

use crate::{Add, Clear, Color, Flag, Format, Plane, private};

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
    pub fn with_content<C2: Display>(&self, content: C2) -> Formatted<C2> {
        Formatted {
            content,
            format: self.format,
        }
    }

    #[must_use]
    pub fn into_content(self) -> C {
        self.content
    }

    #[must_use]
    pub fn get_format(&self) -> Format {
        self.format
    }

    #[must_use]
    pub fn with_format(self, format: Format) -> Formatted<C> {
        Self { format, ..self }
    }
}
impl<C: Display> Add for Formatted<C> {}
impl<C: Display> Clear for Formatted<C> {
    fn set_flag(mut self, flag: Flag, value: bool) -> Self {
        self.format = self.format.set_flag(flag, value);
        self
    }

    fn get_flag(&self, flag: Flag) -> bool {
        self.format.get_flag(flag)
    }

    fn set_color(mut self, plane: Plane, color: Option<Color>) -> Self {
        self.format = self.format.set_color(plane, color);
        self
    }

    fn get_color(&self, plane: Plane) -> Option<Color> {
        self.format.get_color(plane)
    }
}
impl<C: Display> private::ModifyFormat for Formatted<C> {
    fn modify_format(mut self, modify: impl Fn(Format) -> Format) -> Self {
        self.format = modify(self.format);
        self
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
    use crate::{Color, ColorInAPlane, Flag, Plane, assert_display};

    use super::*;

    #[test]
    fn content_and_format() {
        let fmtd = Formatted::new("CONTENT").bold();
        assert_eq!(fmtd.get_content(), &"CONTENT");
        assert_eq!(fmtd.get_format(), Format::new().bold());

        let fmtd = fmtd.bold().with_content("NEW CONTENT");
        assert_eq!(fmtd.get_content(), &"NEW CONTENT");
        assert_eq!(fmtd.get_format(), Format::new().bold());

        let fmtd = fmtd.with_format(Format::new().fg(Color::Red));
        assert_eq!(fmtd.get_content(), &"NEW CONTENT");
        assert_eq!(fmtd.get_format(), Format::new().fg(Color::Red));

        let content = fmtd.into_content();
        assert_eq!(content, "NEW CONTENT");
    }

    #[test]
    fn display_no_format() {
        let fmtd = Formatted::new("CONTENT");
        assert_display!(fmtd, "CONTENT");
    }

    #[test]
    fn flags() {
        let fmtd = Formatted::new("CONTENT");

        assert_display!(fmtd, "CONTENT");
        assert_display!(fmtd.bold(), "\x1b[1mCONTENT\x1b[0m");
        assert_display!(fmtd.faint(), "\x1b[2mCONTENT\x1b[0m");
        assert_display!(fmtd.italic(), "\x1b[3mCONTENT\x1b[0m");
        assert_display!(fmtd.underline(), "\x1b[4mCONTENT\x1b[0m");
        assert_display!(fmtd.slow_blink(), "\x1b[5mCONTENT\x1b[0m");
        assert_display!(fmtd.rapid_blink(), "\x1b[6mCONTENT\x1b[0m");
        assert_display!(fmtd.reverse(), "\x1b[7mCONTENT\x1b[0m");
        assert_display!(fmtd.conceal(), "\x1b[8mCONTENT\x1b[0m");
        assert_display!(fmtd.crossed_out(), "\x1b[9mCONTENT\x1b[0m");
        assert_display!(fmtd.double_underline(), "\x1b[21mCONTENT\x1b[0m");
        assert_display!(fmtd.overline(), "\x1b[53mCONTENT\x1b[0m");

        assert_eq!(fmtd.bold().flag(Flag::Faint), fmtd.bold().faint());
        assert_eq!(fmtd.bold().set_flag(Flag::Bold, false), fmtd);
        assert_eq!(fmtd.bold().set_flag(Flag::Bold, true), fmtd.bold());
        assert_eq!(fmtd.bold().set_flag(Flag::Faint, false), fmtd.bold());
        assert_eq!(fmtd.bold().set_flag(Flag::Faint, true), fmtd.bold().faint());
        assert_eq!(fmtd.bold().get_flag(Flag::Bold), true);
        assert_eq!(fmtd.bold().get_flag(Flag::Faint), false);
    }

    #[test]
    fn fg() {
        let fmtd = Formatted::new("CONTENT");
        assert_eq!(fmtd.get_color(Plane::Foreground), None);

        let fmtd = fmtd.fg(Color::Red);
        assert_display!(fmtd, "\x1b[31mCONTENT\x1b[0m");
        assert_eq!(fmtd.get_color(Plane::Foreground), Some(Color::Red));
    }

    #[test]
    fn bg() {
        let fmtd = Formatted::new("CONTENT");
        assert_eq!(fmtd.get_color(Plane::Background), None);

        let fmtd = fmtd.bg(Color::Red);
        assert_display!(fmtd, "\x1b[41mCONTENT\x1b[0m");
        assert_eq!(fmtd.get_color(Plane::Background), Some(Color::Red));
    }

    #[test]
    fn color() {
        let fmtd = Formatted::new("CONTENT");
        assert_eq!(fmtd.get_color(Plane::Foreground), None);
        assert_eq!(fmtd.get_color(Plane::Background), None);

        let fmtd = fmtd.fg(Color::Red).bg(Color::Green);
        assert_eq!(fmtd.get_color(Plane::Foreground), Some(Color::Red));
        assert_eq!(fmtd.get_color(Plane::Background), Some(Color::Green));

        let fmtd = fmtd
            .color(ColorInAPlane::new(Color::Yellow, Plane::Foreground))
            .color(ColorInAPlane::new(Color::Blue, Plane::Background));
        assert_eq!(fmtd.get_color(Plane::Foreground), Some(Color::Yellow));
        assert_eq!(fmtd.get_color(Plane::Background), Some(Color::Blue));

        let fmtd = fmtd
            .set_color(Plane::Foreground, Some(Color::Magenta))
            .set_color(Plane::Background, None);
        assert_eq!(fmtd.get_color(Plane::Foreground), Some(Color::Magenta));
        assert_eq!(fmtd.get_color(Plane::Background), None);

        let fmtd = fmtd
            .set_color(Plane::Foreground, None)
            .set_color(Plane::Background, Some(Color::Cyan));
        assert_eq!(fmtd.get_color(Plane::Foreground), None);
        assert_eq!(fmtd.get_color(Plane::Background), Some(Color::Cyan));
    }

    #[test]
    fn combined() {
        let fmtd = Formatted::new("CONTENT")
            .bold()
            .fg(Color::Red)
            .underline()
            .bg(Color::Green);
        assert_eq!(
            fmtd.get_format(),
            Format::new()
                .bold()
                .fg(Color::Red)
                .underline()
                .bg(Color::Green)
        );
        assert_display!(fmtd, "\x1b[1;4;31;42mCONTENT\x1b[0m");
    }
}
