use core::fmt::Display;

use crate::{Style, Styled, ToStyle, ToStyleSet};

/// A trait to apply styling to some content.
pub trait AppliedTo: ToStyle + ToStyleSet<StyleSet = Style> {
    /// Applies the styling to the given content, returning a [`Styled<C>`](Styled) instance.
    #[must_use]
    fn applied_to<C: Display>(self, content: C) -> Styled<C> {
        self.to_style().applied_to(content)
    }
}
