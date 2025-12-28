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

#[cfg(test)]
mod tests {
    /// Includes tests for the [`ColorKind`](crate::color::ColorKind) trait methods.
    #[macro_export]
    macro_rules! test_color_kind_methods {
        ($mod:ident; $color:expr, $as_color:expr) => {
            mod $mod {
                $crate::test_color_kind_methods!($color, $as_color);
            }
        };
        ($color:expr, $as_color:expr) => {
            mod color_kind {
                use crate::{color::*, *};

                #[test]
                fn in_fg() {
                    assert_eq!(
                        $color.in_fg(),
                        ColorInAPlane::new($color, Plane::Foreground)
                    );
                    assert_eq!(
                        $color.in_plane(Plane::Foreground),
                        ColorInAPlane::new($color, Plane::Foreground)
                    );
                }

                #[test]
                fn in_bg() {
                    assert_eq!(
                        $color.in_bg(),
                        ColorInAPlane::new($color, Plane::Background)
                    );
                    assert_eq!(
                        $color.in_plane(Plane::Background),
                        ColorInAPlane::new($color, Plane::Background)
                    );
                }

                #[test]
                fn to_color() {
                    assert_eq!($color.to_color(), $as_color);
                }
            }
        };
    }
}
