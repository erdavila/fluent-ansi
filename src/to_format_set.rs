use crate::{Color, ColorInAPlane, Flag, Format, FormatSet};

pub trait FormatElement {
    #[must_use]
    fn add_to_format(self, format: Format) -> Format;
}

pub trait ToFormatSet: Sized {
    type FormatSet: FormatSet;

    #[must_use]
    fn bold(self) -> Self::FormatSet {
        self.flag(Flag::Bold)
    }

    #[must_use]
    fn faint(self) -> Self::FormatSet {
        self.flag(Flag::Faint)
    }

    #[must_use]
    fn italic(self) -> Self::FormatSet {
        self.flag(Flag::Italic)
    }

    #[must_use]
    fn underline(self) -> Self::FormatSet {
        self.flag(Flag::Underline)
    }

    #[must_use]
    fn slow_blink(self) -> Self::FormatSet {
        self.flag(Flag::SlowBlink)
    }

    #[must_use]
    fn rapid_blink(self) -> Self::FormatSet {
        self.flag(Flag::RapidBlink)
    }

    #[must_use]
    fn reverse(self) -> Self::FormatSet {
        self.flag(Flag::Reverse)
    }

    #[must_use]
    fn conceal(self) -> Self::FormatSet {
        self.flag(Flag::Conceal)
    }

    #[must_use]
    fn crossed_out(self) -> Self::FormatSet {
        self.flag(Flag::CrossedOut)
    }

    #[must_use]
    fn double_underline(self) -> Self::FormatSet {
        self.flag(Flag::DoubleUnderline)
    }

    #[must_use]
    fn overline(self) -> Self::FormatSet {
        self.flag(Flag::Overline)
    }

    #[must_use]
    fn flag(self, flag: Flag) -> Self::FormatSet {
        self.add(flag)
    }

    #[must_use]
    fn fg(self, color: impl Into<Color>) -> Self::FormatSet {
        self.color(ColorInAPlane::new_in_fg(color))
    }

    #[must_use]
    fn bg(self, color: impl Into<Color>) -> Self::FormatSet {
        self.color(ColorInAPlane::new_in_bg(color))
    }

    #[must_use]
    fn color(self, color_in_a_plane: ColorInAPlane) -> Self::FormatSet {
        self.add(color_in_a_plane)
    }

    #[must_use]
    fn add(self, element: impl FormatElement) -> Self::FormatSet {
        self.to_format_set().add(element)
    }

    #[must_use]
    fn to_format_set(self) -> Self::FormatSet;
}
