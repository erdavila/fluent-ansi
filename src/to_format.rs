use crate::Format;

pub trait ToFormat: Into<Format> {
    #[must_use]
    fn to_format(self) -> Format;
}
