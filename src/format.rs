use core::fmt::{Display, Formatter, Result, Write};

use crate::{Color, ColorInAPlane, Formatted, Plane, flags::Flag, private::PrivateWithFormat};

pub trait WithFormat: PrivateWithFormat {
    #[must_use]
    fn bold(self) -> Self {
        self.flag(Flag::Bold)
    }

    #[must_use]
    fn faint(self) -> Self {
        self.flag(Flag::Faint)
    }

    #[must_use]
    fn italic(self) -> Self {
        self.flag(Flag::Italic)
    }

    #[must_use]
    fn underline(self) -> Self {
        self.flag(Flag::Underline)
    }

    #[must_use]
    fn slow_blink(self) -> Self {
        self.flag(Flag::SlowBlink)
    }

    #[must_use]
    fn rapid_blink(self) -> Self {
        self.flag(Flag::RapidBlink)
    }

    #[must_use]
    fn reverse(self) -> Self {
        self.flag(Flag::Reverse)
    }

    #[must_use]
    fn conceal(self) -> Self {
        self.flag(Flag::Conceal)
    }

    #[must_use]
    fn crossed_out(self) -> Self {
        self.flag(Flag::CrossedOut)
    }

    #[must_use]
    fn double_underline(self) -> Self {
        self.flag(Flag::DoubleUnderline)
    }

    #[must_use]
    fn overline(self) -> Self {
        self.flag(Flag::Overline)
    }

    #[must_use]
    fn flag(self, flag: Flag) -> Self {
        self.with_flag(flag, true)
    }

    #[must_use]
    fn with_flag(self, flag: Flag, value: bool) -> Self {
        self.modify_format(|fmt| {
            if value {
                fmt.flags |= 1 << flag as u8;
            } else {
                fmt.flags &= !(1 << flag as u8);
            }
        })
    }

    #[must_use]
    fn get_flag(&self, flag: Flag) -> bool {
        self.get_format().flags & (1 << flag as u8) != 0
    }

    #[must_use]
    fn fg(self, color: Color) -> Self {
        self.color(color.fg())
    }

    #[must_use]
    fn bg(self, color: Color) -> Self {
        self.color(color.bg())
    }

    #[must_use]
    fn color(self, color_in_a_plane: ColorInAPlane) -> Self {
        self.with_color(
            Some(color_in_a_plane.get_color()),
            color_in_a_plane.get_plane(),
        )
    }

    #[must_use]
    fn with_color(self, color: Option<Color>, plane: Plane) -> Self {
        self.modify_format(|fmt| match plane {
            Plane::Foreground => fmt.fg = color,
            Plane::Background => fmt.bg = color,
        })
    }

    #[must_use]
    fn get_color(self, plane: Plane) -> Option<Color> {
        match plane {
            Plane::Foreground => self.get_format().fg,
            Plane::Background => self.get_format().bg,
        }
    }
}
// Automatically implement WithFormat for every type that implements PrivateWithFormat
impl<T: PrivateWithFormat> WithFormat for T {}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Format {
    flags: u16,
    fg: Option<Color>,
    bg: Option<Color>,
}
impl Format {
    #[must_use]
    pub fn new() -> Self {
        Format::default()
    }

    #[must_use]
    pub fn applied_to<C: Display>(self, content: C) -> Formatted<C> {
        Formatted::new(content).with_format(self)
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
                    let mut any = false;
                    let mut write_code = |code| {
                        if any {
                            f.write_char(';')?;
                        }
                        write!(f, "{code}")?;
                        any = true;
                        Ok(())
                    };

                    for flag in enum_iterator::all::<Flag>() {
                        if self.0.get_flag(flag) {
                            write_code(flag.get_code())?;
                        }
                    }
                    if let Some(color) = self.0.fg {
                        write_code(30 + color as u8)?;
                    }
                    if let Some(color) = self.0.bg {
                        write_code(40 + color as u8)?;
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

fn write_escape_sequence(f: &mut impl Write, codes: impl Display) -> Result {
    write!(f, "\x1b[{codes}m")
}

#[cfg(test)]
mod tests {
    use crate::assert_display;

    use super::*;

    #[test]
    fn flags() {
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
        assert_eq!(bold_format.with_flag(Flag::Bold, false), fmt);
        assert_eq!(bold_format.with_flag(Flag::Bold, true), fmt.bold());
        assert_eq!(bold_format.with_flag(Flag::Faint, false), fmt.bold());
        assert_eq!(bold_format.with_flag(Flag::Faint, true), fmt.bold().faint());
        assert_eq!(bold_format.get_flag(Flag::Bold), true);
        assert_eq!(bold_format.get_flag(Flag::Faint), false);
    }

    #[test]
    fn fg() {
        let fmt = Format::new();
        assert_eq!(fmt.get_color(Plane::Foreground), None);

        let fmt = fmt.fg(Color::Red);
        assert_display!(fmt, "\x1b[31m");
        assert_eq!(fmt.get_color(Plane::Foreground), Some(Color::Red));
    }

    #[test]
    fn bg() {
        let fmt = Format::new();
        assert_eq!(fmt.get_color(Plane::Background), None);

        let fmt = fmt.bg(Color::Red);
        assert_display!(fmt, "\x1b[41m");
        assert_eq!(fmt.get_color(Plane::Background), Some(Color::Red));
    }

    #[test]
    fn color() {
        let fmt = Format::new();
        assert_eq!(fmt.get_color(Plane::Foreground), None);
        assert_eq!(fmt.get_color(Plane::Background), None);

        let fmt = fmt.fg(Color::Red).bg(Color::Green);
        assert_eq!(fmt.get_color(Plane::Foreground), Some(Color::Red));
        assert_eq!(fmt.get_color(Plane::Background), Some(Color::Green));

        let fmt = fmt
            .color(ColorInAPlane::new(Color::Yellow, Plane::Foreground))
            .color(ColorInAPlane::new(Color::Blue, Plane::Background));
        assert_eq!(fmt.get_color(Plane::Foreground), Some(Color::Yellow));
        assert_eq!(fmt.get_color(Plane::Background), Some(Color::Blue));

        let fmt = fmt
            .with_color(Some(Color::Magenta), Plane::Foreground)
            .with_color(None, Plane::Background);
        assert_eq!(fmt.get_color(Plane::Foreground), Some(Color::Magenta));
        assert_eq!(fmt.get_color(Plane::Background), None);

        let fmt = fmt
            .with_color(None, Plane::Foreground)
            .with_color(Some(Color::Cyan), Plane::Background);
        assert_eq!(fmt.get_color(Plane::Foreground), None);
        assert_eq!(fmt.get_color(Plane::Background), Some(Color::Cyan));
    }

    #[test]
    fn combined() {
        let fmt = Format::new()
            .bold()
            .fg(Color::Red)
            .underline()
            .bg(Color::Green);
        assert_display!(fmt, "\x1b[1;4;31;42m");
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
    fn from_flag() {
        assert_eq!(Format::from(Flag::Bold), Format::new().bold());
    }

    #[test]
    fn from_color_in_a_plane() {
        assert_eq!(
            Format::from(Color::Red.fg()),
            Format::new().color(Color::Red.fg())
        );
    }
}
