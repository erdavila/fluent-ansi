use core::fmt::{Display, Formatter, Result, Write};

use crate::{Color, ColorInAPlane, Plane, private::PrivateWithFormat};

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
    bold: bool,
    fg: Option<Color>,
    bg: Option<Color>,
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
                    let mut any = false;
                    let mut write_code = |code| {
                        if any {
                            f.write_char(';')?;
                        }
                        write!(f, "{code}")?;
                        any = true;
                        Ok(())
                    };

                    if self.0.bold {
                        write_code(1)?;
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
        let fmt = Format::new().bold().fg(Color::Red).bg(Color::Green);
        assert_display!(fmt, "\x1b[1;31;42m");
    }

    #[test]
    fn default() {
        assert_display!(Format::default(), "\x1b[0m");
    }

    #[test]
    fn from_color_in_a_plane() {
        assert_eq!(
            Format::from(Color::Red.fg()),
            Format::new().color(Color::Red.fg())
        );
    }
}
