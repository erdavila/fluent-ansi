use core::fmt::Result;

use crate::{CodeWriter, ColorKind, Plane, WriteColorCodes};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SimpleColor {
    basic_color: BasicColor,
    bright: bool,
}

impl SimpleColor {
    #[must_use]
    pub fn new(basic_color: BasicColor) -> Self {
        Self {
            basic_color,
            bright: false,
        }
    }

    #[must_use]
    pub fn new_bright(basic_color: BasicColor) -> Self {
        Self::new(basic_color).bright()
    }

    #[must_use]
    pub fn bright(self) -> Self {
        Self {
            bright: true,
            ..self
        }
    }

    #[must_use]
    pub fn get_basic_color(self) -> BasicColor {
        self.basic_color
    }

    #[must_use]
    pub fn is_bright(self) -> bool {
        self.bright
    }
}

impl ColorKind for SimpleColor {}

impl WriteColorCodes for SimpleColor {
    fn write_color_codes(self, plane: Plane, writer: &mut CodeWriter) -> Result {
        let code_base = match (plane, self.bright) {
            (Plane::Foreground, false) => 30,
            (Plane::Background, false) => 40,
            (Plane::Foreground, true) => 90,
            (Plane::Background, true) => 100,
        };

        writer.write_code(code_base + self.basic_color.code_offset())
    }
}

impl From<BasicColor> for SimpleColor {
    fn from(basic_color: BasicColor) -> Self {
        SimpleColor {
            basic_color,
            bright: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BasicColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}
impl BasicColor {
    #[must_use]
    pub fn to_simple_color(self) -> SimpleColor {
        self.into()
    }

    #[must_use]
    pub fn bright(self) -> SimpleColor {
        SimpleColor::new_bright(self)
    }

    #[must_use]
    fn code_offset(self) -> u8 {
        self as u8
    }
}

impl ColorKind for BasicColor {}

#[cfg(test)]
mod tests {
    use crate::{Color, ColorInAPlane, Plane};

    use super::*;

    #[test]
    fn simple_new() {
        let color = SimpleColor::new(BasicColor::Red);

        assert_eq!(
            color,
            SimpleColor {
                basic_color: BasicColor::Red,
                bright: false
            }
        );
        assert_eq!(color.get_basic_color(), BasicColor::Red);
        assert_eq!(color.is_bright(), false);
    }

    #[test]
    fn simple_new_bright() {
        let color = SimpleColor::new_bright(BasicColor::Red);

        assert_eq!(
            color,
            SimpleColor {
                basic_color: BasicColor::Red,
                bright: true
            }
        );
        assert_eq!(color.get_basic_color(), BasicColor::Red);
        assert_eq!(color.is_bright(), true);
    }

    #[test]
    fn simple_bright() {
        let simple_regular_color = SimpleColor::new(BasicColor::Red);
        let simple_bright_color = SimpleColor::new_bright(BasicColor::Red);

        assert_eq!(simple_regular_color.bright(), simple_bright_color);
        assert_eq!(simple_bright_color.bright(), simple_bright_color);
    }

    #[test]
    fn simple_to_color() {
        let simple_color = SimpleColor::new(BasicColor::Red);
        assert_eq!(
            simple_color.to_color(),
            Color::Simple(SimpleColor::new(BasicColor::Red))
        );
    }

    #[test]
    fn simple_from_basic() {
        assert_eq!(
            SimpleColor::from(BasicColor::Red),
            SimpleColor {
                basic_color: BasicColor::Red,
                bright: false
            }
        );
    }

    #[test]
    fn basic_bright() {
        assert_eq!(
            BasicColor::Red.bright(),
            SimpleColor {
                basic_color: BasicColor::Red,
                bright: true
            }
        );
    }

    #[test]
    fn basic_in_fg() {
        assert_eq!(
            BasicColor::Red.in_fg(),
            ColorInAPlane::new(BasicColor::Red, Plane::Foreground)
        );
        assert_eq!(
            BasicColor::Red.in_plane(Plane::Foreground),
            ColorInAPlane::new(BasicColor::Red, Plane::Foreground)
        );
    }

    #[test]
    fn basic_in_bg() {
        assert_eq!(
            BasicColor::Red.in_bg(),
            ColorInAPlane::new(BasicColor::Red, Plane::Background)
        );
        assert_eq!(
            BasicColor::Red.in_plane(Plane::Background),
            ColorInAPlane::new(BasicColor::Red, Plane::Background)
        );
    }

    #[test]
    fn basic_to_simple_color() {
        assert_eq!(
            BasicColor::Red.to_simple_color(),
            SimpleColor {
                basic_color: BasicColor::Red,
                bright: false
            }
        );
    }

    #[test]
    fn basic_to_color() {
        assert_eq!(
            BasicColor::Red.to_color(),
            Color::Simple(SimpleColor {
                basic_color: BasicColor::Red,
                bright: false
            })
        );
    }
}
