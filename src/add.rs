use crate::{Clear, Color, ColorInAPlane, Flag, private};

pub trait Add: private::ModifyFormat + Sized {
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
        self.modify_format(|fmt| fmt.set_flag(flag, true))
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
        self.modify_format(|fmt| {
            fmt.set_color(
                color_in_a_plane.get_plane(),
                Some(color_in_a_plane.get_color()),
            )
        })
    }
}
