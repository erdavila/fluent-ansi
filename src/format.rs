use core::fmt::{Display, Formatter, Result, Write};

use crate::{
    AppliedTo, ColorInAPlane, Flag, FormatElement, FormatSet, Formatted, Plane, Reset, ToFormat,
    ToFormatSet,
    color::{Color, WriteColorCodes as _},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Format {
    pub(crate) flags: u16,
    pub(crate) fg: Option<Color>,
    pub(crate) bg: Option<Color>,
}

impl Format {
    #[must_use]
    pub fn new() -> Self {
        Format::default()
    }
}

impl ToFormatSet for Format {
    type FormatSet = Self;

    fn add(self, element: impl FormatElement) -> Self::FormatSet {
        element.add_to_format(self)
    }

    fn to_format_set(self) -> Self::FormatSet {
        self
    }
}

impl ToFormat for Format {
    fn to_format(self) -> Format {
        self
    }
}

impl AppliedTo for Format {
    fn applied_to<C: Display>(self, content: C) -> Formatted<C> {
        Formatted::new(content).with_format(self)
    }
}

impl FormatSet for Format {
    fn set<P: crate::Position>(self, position: P, value: P::Value) -> Self {
        position.set_in_format(self, value)
    }

    fn get<P: crate::Position>(&self, position: P) -> P::Value {
        position.get_from_format(self)
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
                    let mut code_writer = CodeWriter { f, any: false };

                    for flag in enum_iterator::all::<Flag>() {
                        if self.0.get_flag(flag) {
                            code_writer.write_code(flag.get_code())?;
                        }
                    }
                    if let Some(color) = self.0.fg {
                        color.write_color_codes(Plane::Foreground, &mut code_writer)?;
                    }
                    if let Some(color) = self.0.bg {
                        color.write_color_codes(Plane::Background, &mut code_writer)?;
                    }
                    Ok(())
                }
            }
            write_escape_sequence(f, Codes(*self))
        }
    }
}

impl From<Flag> for Format {
    fn from(flag: Flag) -> Self {
        Format::new().flag(flag)
    }
}

impl From<ColorInAPlane> for Format {
    fn from(color_in_a_plane: ColorInAPlane) -> Self {
        Format::new().color(color_in_a_plane)
    }
}

impl From<Reset> for Format {
    fn from(_: Reset) -> Self {
        Format::new()
    }
}

impl PartialEq<Reset> for Format {
    fn eq(&self, other: &Reset) -> bool {
        *self == other.to_format()
    }
}

pub(crate) struct CodeWriter<'a, 'b> {
    f: &'a mut Formatter<'b>,
    any: bool,
}

impl CodeWriter<'_, '_> {
    pub(crate) fn write_code(&mut self, code: u8) -> Result {
        if self.any {
            self.f.write_char(';')?;
        }
        write!(self.f, "{code}")?;
        self.any = true;
        Ok(())
    }
}

fn write_escape_sequence(f: &mut impl Write, codes: impl Display) -> Result {
    write!(f, "\x1b[{codes}m")
}

#[cfg(test)]
mod tests {
    use crate::{
        Plane, assert_display,
        color::{BasicColor, ColorKind as _, SimpleColor},
    };

    use super::*;

    #[test]
    fn flag() {
        let fmt = Format::new();

        assert_display!(fmt, "\x1b[0m");
        assert_display!(fmt.bold(), "\x1b[1m");
        assert_display!(fmt.faint(), "\x1b[2m");
        assert_display!(fmt.italic(), "\x1b[3m");
        assert_display!(fmt.underline(), "\x1b[4m");
        assert_display!(fmt.slow_blink(), "\x1b[5m");
        assert_display!(fmt.rapid_blink(), "\x1b[6m");
        assert_display!(fmt.reverse(), "\x1b[7m");
        assert_display!(fmt.conceal(), "\x1b[8m");
        assert_display!(fmt.crossed_out(), "\x1b[9m");
        assert_display!(fmt.double_underline(), "\x1b[21m");
        assert_display!(fmt.overline(), "\x1b[53m");

        let bold_format = fmt.bold();
        assert_eq!(bold_format.flag(Flag::Faint), fmt.bold().faint());
        assert_eq!(bold_format.add(Flag::Faint), fmt.bold().faint());
        assert_eq!(bold_format.set_flag(Flag::Bold, false), fmt);
        assert_eq!(bold_format.set_flag(Flag::Bold, true), fmt.bold());
        assert_eq!(bold_format.set_flag(Flag::Faint, false), fmt.bold());
        assert_eq!(bold_format.set_flag(Flag::Faint, true), fmt.bold().faint());
        assert_eq!(bold_format.get_flag(Flag::Bold), true);
        assert_eq!(bold_format.get_flag(Flag::Faint), false);
        assert_eq!(bold_format.set(Flag::Bold, false), fmt);
        assert_eq!(bold_format.set(Flag::Bold, true), fmt.bold());
        assert_eq!(bold_format.set(Flag::Faint, false), fmt.bold());
        assert_eq!(bold_format.set(Flag::Faint, true), fmt.bold().faint());
        assert_eq!(bold_format.get(Flag::Bold), true);
        assert_eq!(bold_format.get(Flag::Faint), false);
        assert_eq!(bold_format.unset(Flag::Bold), fmt);
        assert_eq!(bold_format.unset(Flag::Faint), fmt.bold());
    }

    #[test]
    fn fg() {
        let fmt = Format::new();
        assert_eq!(fmt.get_color(Plane::Foreground), None);

        let fmt = fmt.fg(BasicColor::Red);
        assert_display!(fmt, "\x1b[31m");
        assert_eq!(
            fmt.get_color(Plane::Foreground),
            Some(Color::Simple(SimpleColor::new(BasicColor::Red)))
        );
    }

    #[test]
    fn bg() {
        let fmt = Format::new();
        assert_eq!(fmt.get_color(Plane::Background), None);

        let fmt = fmt.bg(BasicColor::Red);
        assert_display!(fmt, "\x1b[41m");
        assert_eq!(
            fmt.get_color(Plane::Background),
            Some(Color::Simple(SimpleColor::new(BasicColor::Red)))
        );
    }

    #[test]
    fn color() {
        let fmt = Format::new();
        assert_eq!(fmt.get_color(Plane::Foreground), None);
        assert_eq!(fmt.get_color(Plane::Background), None);

        let fmt = Format::new().fg(BasicColor::Red).bg(BasicColor::Green);
        assert_eq!(
            fmt.get_color(Plane::Foreground),
            Some(Color::Simple(SimpleColor::new(BasicColor::Red)))
        );
        assert_eq!(
            fmt.get_color(Plane::Background),
            Some(Color::Simple(SimpleColor::new(BasicColor::Green)))
        );

        let fmt = Format::new()
            .color(ColorInAPlane::new(BasicColor::Yellow, Plane::Foreground))
            .color(ColorInAPlane::new(BasicColor::Blue, Plane::Background));
        assert_eq!(
            fmt.get_color(Plane::Foreground),
            Some(Color::Simple(SimpleColor::new(BasicColor::Yellow)))
        );
        assert_eq!(
            fmt.get_color(Plane::Background),
            Some(Color::Simple(SimpleColor::new(BasicColor::Blue)))
        );

        let fmt = Format::new()
            .add(ColorInAPlane::new(BasicColor::White, Plane::Foreground))
            .add(ColorInAPlane::new(BasicColor::Black, Plane::Background));
        assert_eq!(
            fmt.get_color(Plane::Foreground),
            Some(Color::Simple(SimpleColor::new(BasicColor::White)))
        );
        assert_eq!(
            fmt.get_color(Plane::Background),
            Some(Color::Simple(SimpleColor::new(BasicColor::Black)))
        );

        let fmt = fmt
            .set_color(Plane::Foreground, Some(BasicColor::Magenta))
            .set_color(Plane::Background, None::<Color>);
        assert_eq!(
            fmt.get_color(Plane::Foreground),
            Some(Color::Simple(SimpleColor::new(BasicColor::Magenta)))
        );
        assert_eq!(fmt.get_color(Plane::Background), None);

        let fmt = fmt
            .set_color(Plane::Foreground, None::<Color>)
            .set_color(Plane::Background, Some(BasicColor::Cyan));
        assert_eq!(fmt.get_color(Plane::Foreground), None);
        assert_eq!(
            fmt.get_color(Plane::Background),
            Some(Color::Simple(SimpleColor::new(BasicColor::Cyan)))
        );

        let fmt = fmt
            .set(Plane::Foreground, Some(BasicColor::Magenta.to_color()))
            .set(Plane::Background, None);
        assert_eq!(
            fmt.get(Plane::Foreground),
            Some(BasicColor::Magenta.to_color())
        );
        assert_eq!(fmt.get(Plane::Background), None);

        let fmt = fmt
            .set(Plane::Foreground, None)
            .set(Plane::Background, Some(BasicColor::Cyan.to_color()));
        assert_eq!(fmt.get(Plane::Foreground), None);
        assert_eq!(
            fmt.get(Plane::Background),
            Some(BasicColor::Cyan.to_color())
        );

        let fmt = fmt.unset(Plane::Background);
        assert_eq!(fmt.get(Plane::Foreground), None);
        assert_eq!(fmt.get(Plane::Background), None);
    }

    #[test]
    fn combined() {
        let fmt = Format::new()
            .bold()
            .fg(BasicColor::Red)
            .underline()
            .bg(BasicColor::Green);
        assert_display!(fmt, "\x1b[1;4;31;42m");
        assert_eq!(
            fmt.unset(Flag::Bold).unset(Plane::Background),
            Format::new().underline().fg(BasicColor::Red)
        )
    }

    #[test]
    fn applied_to() {
        let fmtd = Format::new().bold().applied_to("CONTENT");

        assert_eq!(fmtd.get_content(), &"CONTENT");
        assert_eq!(fmtd.get_format(), Format::new().bold());
    }

    #[test]
    fn default() {
        assert_display!(Format::default(), "\x1b[0m");
    }

    #[test]
    fn to_format_set() {
        let fmt = Format::new().bold().fg(BasicColor::Red);
        assert_eq!(fmt.to_format_set(), fmt);
    }

    #[test]
    fn to_format() {
        let fmt = Format::new().bold().fg(BasicColor::Red);
        assert_eq!(fmt.to_format(), fmt);
    }

    #[test]
    fn from_flag() {
        assert_eq!(Format::from(Flag::Bold), Format::new().bold());
    }

    #[test]
    fn from_color_in_a_plane() {
        assert_eq!(
            Format::from(BasicColor::Red.in_fg()),
            Format::new().color(BasicColor::Red.in_fg())
        );
    }

    #[test]
    fn from_reset() {
        assert_eq!(Format::from(Reset), Format::new());
    }
}
