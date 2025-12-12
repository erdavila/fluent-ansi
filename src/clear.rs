use crate::{Color, Flag, Plane, ToFormatSet};

pub trait Clear: ToFormatSet<FormatSet = Self> {
    #[must_use]
    fn set_flag(self, flag: Flag, value: bool) -> Self;

    #[must_use]
    fn get_flag(&self, flag: Flag) -> bool;

    #[must_use]
    fn set_color(self, plane: Plane, color: Option<Color>) -> Self;

    #[must_use]
    fn get_color(&self, plane: Plane) -> Option<Color>;
}
