use crate::{Flag, Format, Plane, ToFormatSet, color::Color};

pub trait FormatAttribute {
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
    fn set<A: FormatAttribute>(self, attr: A, value: A::Value) -> Self;

    #[must_use]
    fn get<A: FormatAttribute>(&self, attr: A) -> A::Value;

    #[must_use]
    fn unset<A: FormatAttribute>(self, attr: A) -> Self {
        self.set(attr, A::Value::default())
    }
}
