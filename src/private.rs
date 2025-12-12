use crate::Clear;

pub trait ToFormatSet<FormatSet: Clear> {
    #[must_use]
    fn to_format_set(self) -> FormatSet;
}
