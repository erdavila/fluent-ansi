use crate::{ColorInAPlane, Plane};

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
    pub fn fg(self) -> ColorInAPlane {
        self.in_plane(Plane::Foreground)
    }

    #[must_use]
    pub fn bg(self) -> ColorInAPlane {
        self.in_plane(Plane::Background)
    }

    #[must_use]
    pub fn in_plane(self, plane: Plane) -> ColorInAPlane {
        ColorInAPlane::new(self, plane)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_fg() {
        assert_eq!(
            Color::Red.fg(),
            ColorInAPlane::new(Color::Red, Plane::Foreground)
        );
        assert_eq!(
            Color::Red.in_plane(Plane::Foreground),
            ColorInAPlane::new(Color::Red, Plane::Foreground)
        );
    }

    #[test]
    fn color_bg() {
        assert_eq!(
            Color::Red.bg(),
            ColorInAPlane::new(Color::Red, Plane::Background)
        );
        assert_eq!(
            Color::Red.in_plane(Plane::Background),
            ColorInAPlane::new(Color::Red, Plane::Background)
        );
    }
}
