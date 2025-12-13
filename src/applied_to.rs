use core::fmt::Display;

use crate::{Format, Formatted, ToFormat, ToFormatSet};

pub trait AppliedTo: ToFormat + ToFormatSet<FormatSet = Format> {
    #[must_use]
    fn applied_to<C: Display>(self, content: C) -> Formatted<C> {
        self.to_format().applied_to(content)
    }
}
