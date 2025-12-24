use core::fmt::Result;

use crate::{CodeWriter, ColorInAPlane, Plane, color::Color};

/// A trait for color kinds that can be converted into a [`Color`].
pub trait ColorKind: Into<Color> {
    /// Associate this color with the foreground plane.
    #[must_use]
    fn in_fg(self) -> ColorInAPlane {
        self.in_plane(Plane::Foreground)
    }

    /// Associate this color with the background plane.
    #[must_use]
    fn in_bg(self) -> ColorInAPlane {
        self.in_plane(Plane::Background)
    }

    /// Associate this color with the specified plane.
    #[must_use]
    fn in_plane(self, plane: Plane) -> ColorInAPlane {
        ColorInAPlane::new(self, plane)
    }

    /// Convert this color kind into a [`Color`].
    #[must_use]
    fn to_color(self) -> Color {
        self.into()
    }
}

pub(crate) trait WriteColorCodes: ColorKind {
    fn write_color_codes(self, plane: Plane, writer: &mut CodeWriter) -> Result;
}

impl<C: Into<Color>> ColorKind for C {}
