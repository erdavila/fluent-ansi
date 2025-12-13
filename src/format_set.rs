use crate::{Color, Flag, Format, Plane, ToFormatSet};

pub trait Position {
    type Value: Default;

    #[must_use]
    fn set_in_format(self, format: Format, value: Self::Value) -> Format;

    #[must_use]
    fn get_from_format(self, format: &Format) -> Self::Value;
}

pub trait FormatSet: ToFormatSet<FormatSet = Self> {
    #[must_use]
    fn set_flag(self, flag: Flag, value: bool) -> Self {
        self.set(flag, value)
    }

    #[must_use]
    fn get_flag(&self, flag: Flag) -> bool {
        self.get(flag)
    }

    #[must_use]
    fn set_color(self, plane: Plane, color: Option<impl Into<Color>>) -> Self {
        let color: Option<Color> = color.map(Into::into);
        self.set(plane, color)
    }

    #[must_use]
    fn get_color(&self, plane: Plane) -> Option<Color> {
        self.get(plane)
    }

    #[must_use]
    fn set<P: Position>(self, position: P, value: P::Value) -> Self;

    #[must_use]
    fn get<P: Position>(&self, position: P) -> P::Value;

    #[must_use]
    fn unset<P: Position>(self, position: P) -> Self {
        self.set(position, P::Value::default())
    }
}
