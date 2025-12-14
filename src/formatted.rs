use core::fmt::{Display, Formatter, Result};

use crate::{Format, FormatElement, FormatSet, ToFormatSet};

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
impl<C: Display> ToFormatSet for Formatted<C> {
    type FormatSet = Self;

    fn add(self, element: impl FormatElement) -> Self::FormatSet {
        let format = self.format.add(element);
        self.with_format(format)
    }

    fn to_format_set(self) -> Self::FormatSet {
        self
    }
}
impl<C: Display> FormatSet for Formatted<C> {
    fn set<A: crate::FormatAttribute>(self, attr: A, value: A::Value) -> Self {
        let format = self.format.set(attr, value);
        self.with_format(format)
    }

    fn get<A: crate::FormatAttribute>(&self, attr: A) -> A::Value {
        self.format.get(attr)
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
    use crate::{
        ColorInAPlane, Flag, Plane, assert_display,
        color::{BasicColor, Color, ColorKind as _},
    };

    use super::*;

    #[test]
    fn content_and_format() {
        let fmtd = Formatted::new("CONTENT").bold();
        assert_eq!(fmtd.get_content(), &"CONTENT");
        assert_eq!(fmtd.get_format(), Format::new().bold());

        let fmtd = fmtd.bold().with_content("NEW CONTENT");
        assert_eq!(fmtd.get_content(), &"NEW CONTENT");
        assert_eq!(fmtd.get_format(), Format::new().bold());

        let fmtd = fmtd.with_format(Format::new().fg(BasicColor::Red));
        assert_eq!(fmtd.get_content(), &"NEW CONTENT");
        assert_eq!(fmtd.get_format(), Format::new().fg(BasicColor::Red));

        let content = fmtd.into_content();
        assert_eq!(content, "NEW CONTENT");
    }

    #[test]
    fn display_no_format() {
        let fmtd = Formatted::new("CONTENT");
        assert_display!(fmtd, "CONTENT");
    }

    #[test]
    fn add_flag() {
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

        let bold_fmtd = fmtd.bold();
        assert_eq!(bold_fmtd.flag(Flag::Faint), fmtd.bold().faint());
        assert_eq!(bold_fmtd.add(Flag::Faint), fmtd.bold().faint());
        assert_eq!(bold_fmtd.set_flag(Flag::Bold, false), fmtd);
        assert_eq!(bold_fmtd.set_flag(Flag::Bold, true), fmtd.bold());
        assert_eq!(bold_fmtd.set_flag(Flag::Faint, false), fmtd.bold());
        assert_eq!(bold_fmtd.set_flag(Flag::Faint, true), fmtd.bold().faint());
        assert_eq!(bold_fmtd.get_flag(Flag::Bold), true);
        assert_eq!(bold_fmtd.get_flag(Flag::Faint), false);
        assert_eq!(bold_fmtd.set(Flag::Bold, false), fmtd);
        assert_eq!(bold_fmtd.set(Flag::Bold, true), fmtd.bold());
        assert_eq!(bold_fmtd.set(Flag::Faint, false), fmtd.bold());
        assert_eq!(bold_fmtd.set(Flag::Faint, true), fmtd.bold().faint());
        assert_eq!(bold_fmtd.get(Flag::Bold), true);
        assert_eq!(bold_fmtd.get(Flag::Faint), false);
        assert_eq!(bold_fmtd.unset(Flag::Bold), fmtd);
        assert_eq!(bold_fmtd.unset(Flag::Faint), fmtd.bold());
    }

    #[test]
    fn fg() {
        let fmtd = Formatted::new("CONTENT");
        assert_eq!(fmtd.get_color(Plane::Foreground), None);

        let fmtd = fmtd.fg(BasicColor::Red);
        assert_display!(fmtd, "\x1b[31mCONTENT\x1b[0m");
        assert_eq!(
            fmtd.get_color(Plane::Foreground),
            Some(BasicColor::Red.to_color())
        );
    }

    #[test]
    fn bg() {
        let fmtd = Formatted::new("CONTENT");
        assert_eq!(fmtd.get_color(Plane::Background), None);

        let fmtd = fmtd.bg(BasicColor::Red);
        assert_display!(fmtd, "\x1b[41mCONTENT\x1b[0m");
        assert_eq!(
            fmtd.get_color(Plane::Background),
            Some(BasicColor::Red.to_color())
        );
    }

    #[test]
    fn add_color() {
        let fmtd_base = Formatted::new("CONTENT");
        assert_eq!(fmtd_base.get_color(Plane::Foreground), None);
        assert_eq!(fmtd_base.get_color(Plane::Background), None);

        let fmtd = fmtd_base.fg(BasicColor::Red).bg(BasicColor::Green);
        assert_eq!(
            fmtd.get_color(Plane::Foreground),
            Some(BasicColor::Red.to_color())
        );
        assert_eq!(
            fmtd.get_color(Plane::Background),
            Some(BasicColor::Green.to_color())
        );

        let fmtd = fmtd_base
            .color(ColorInAPlane::new(BasicColor::Yellow, Plane::Foreground))
            .color(ColorInAPlane::new(BasicColor::Blue, Plane::Background));
        assert_eq!(
            fmtd.get_color(Plane::Foreground),
            Some(BasicColor::Yellow.to_color())
        );
        assert_eq!(
            fmtd.get_color(Plane::Background),
            Some(BasicColor::Blue.to_color())
        );

        let fmtd = fmtd_base
            .add(ColorInAPlane::new(BasicColor::White, Plane::Foreground))
            .add(ColorInAPlane::new(BasicColor::Black, Plane::Background));
        assert_eq!(
            fmtd.get_color(Plane::Foreground),
            Some(BasicColor::White.to_color())
        );
        assert_eq!(
            fmtd.get_color(Plane::Background),
            Some(BasicColor::Black.to_color())
        );

        let fmtd = fmtd
            .set_color(Plane::Foreground, Some(BasicColor::Magenta))
            .set_color(Plane::Background, None::<Color>);
        assert_eq!(
            fmtd.get_color(Plane::Foreground),
            Some(BasicColor::Magenta.to_color())
        );
        assert_eq!(fmtd.get_color(Plane::Background), None);

        let fmtd = fmtd
            .set_color(Plane::Foreground, None::<Color>)
            .set_color(Plane::Background, Some(BasicColor::Cyan));
        assert_eq!(fmtd.get_color(Plane::Foreground), None);
        assert_eq!(
            fmtd.get_color(Plane::Background),
            Some(BasicColor::Cyan.to_color())
        );

        let fmtd = fmtd
            .set(Plane::Foreground, Some(BasicColor::Magenta.to_color()))
            .set(Plane::Background, None);
        assert_eq!(
            fmtd.get(Plane::Foreground),
            Some(BasicColor::Magenta.to_color())
        );
        assert_eq!(fmtd.get(Plane::Background), None);

        let fmtd = fmtd
            .set(Plane::Foreground, None)
            .set(Plane::Background, Some(BasicColor::Cyan.to_color()));
        assert_eq!(fmtd.get(Plane::Foreground), None);
        assert_eq!(
            fmtd.get(Plane::Background),
            Some(BasicColor::Cyan.to_color())
        );

        let fmtd = fmtd.unset(Plane::Background);
        assert_eq!(fmtd.get(Plane::Foreground), None);
        assert_eq!(fmtd.get(Plane::Background), None);
    }

    #[test]
    fn combined() {
        let fmtd = Formatted::new("CONTENT")
            .bold()
            .fg(BasicColor::Red)
            .underline()
            .bg(BasicColor::Green);
        assert_eq!(
            fmtd.get_format(),
            Format::new()
                .bold()
                .fg(BasicColor::Red)
                .underline()
                .bg(BasicColor::Green)
        );
        assert_display!(fmtd, "\x1b[1;4;31;42mCONTENT\x1b[0m");
        assert_eq!(
            fmtd.unset(Flag::Bold).unset(Plane::Background).get_format(),
            Format::new().underline().fg(BasicColor::Red)
        )
    }

    #[test]
    fn to_format_set() {
        let fmtd = Formatted::new("CONTENT").bold().fg(BasicColor::Red);
        assert_eq!(fmtd.to_format_set(), fmtd);
    }
}
