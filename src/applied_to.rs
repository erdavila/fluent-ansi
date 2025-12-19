use core::fmt::Display;

use crate::{Format, Formatted, ToFormat, ToFormatSet};

/// A trait to apply a formatting to some content.
pub trait AppliedTo: ToFormat + ToFormatSet<FormatSet = Format> {
    /// Applies the formatting to the given content, returning a [`Formatted<C>`](Formatted) instance.
    #[must_use]
    fn applied_to<C: Display>(self, content: C) -> Formatted<C> {
        self.to_format().applied_to(content)
    }
}
