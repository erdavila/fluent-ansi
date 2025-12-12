use core::fmt::{Display, Formatter, Result, Write};

use crate::{
    Color, ColorInAPlane, FormatElement, FormatSet, Formatted, Plane, ToFormatSet, flags::Flag,
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

    #[must_use]
    pub fn applied_to<C: Display>(self, content: C) -> Formatted<C> {
        Formatted::new(content).with_format(self)
    }

    pub(crate) fn set_flags_bit(&mut self, flag: Flag) {
        self.flags |= 1 << flag as u8;
    }

    pub(crate) fn clear_flags_bit(&mut self, flag: Flag) {
        self.flags &= !(1 << flag as u8);
    }

    pub(crate) fn get_flags_bit(self, flag: Flag) -> bool {
        self.flags & (1 << flag as u8) != 0
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
impl FormatSet for Format {
    fn set_flag(mut self, flag: Flag, value: bool) -> Self {
        if value {
            self.set_flags_bit(flag);
        } else {
            self.clear_flags_bit(flag);
        }
        self
    }

    fn get_flag(&self, flag: Flag) -> bool {
        self.get_flags_bit(flag)
    }

    fn set_color(mut self, plane: Plane, color: Option<Color>) -> Self {
        match plane {
            Plane::Foreground => self.fg = color,
            Plane::Background => self.bg = color,
        }
        self
    }

    fn get_color(&self, plane: Plane) -> Option<Color> {
        match plane {
            Plane::Foreground => self.fg,
            Plane::Background => self.bg,
        }
    }
}
impl Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if *self == Format::new() {
            write!(f, "{Reset}")
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Reset;

impl Display for Reset {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write_escape_sequence(f, 0)
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
    fn add_flag() {
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
    fn add_color() {
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
            .add(ColorInAPlane::new(Color::White, Plane::Foreground))
            .add(ColorInAPlane::new(Color::Black, Plane::Background));
        assert_eq!(fmt.get_color(Plane::Foreground), Some(Color::White));
        assert_eq!(fmt.get_color(Plane::Background), Some(Color::Black));

        let fmt = fmt
            .set_color(Plane::Foreground, Some(Color::Magenta))
            .set_color(Plane::Background, None);
        assert_eq!(fmt.get_color(Plane::Foreground), Some(Color::Magenta));
        assert_eq!(fmt.get_color(Plane::Background), None);

        let fmt = fmt
            .set_color(Plane::Foreground, None)
            .set_color(Plane::Background, Some(Color::Cyan));
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
    fn to_format_set() {
        let fmt = Format::new().bold().fg(Color::Red);
        assert_eq!(fmt.to_format_set(), fmt);
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

    #[test]
    fn reset() {
        assert_display!(Reset, "\x1b[0m");
    }
}
