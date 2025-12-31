use core::fmt::{Display, Formatter, Result};

use crate::Style;

/// A type that represents the reset of all styling.
///
/// When rendered, it produces the ANSI escape sequence to reset all styling.
///
/// It is equal to a [`Style::new()`].
///
/// See [The `Reset` singleton](crate#the-reset-singleton).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Reset;

impl Reset {
    /// Converts the type into a [`Style`].
    #[must_use]
    pub fn to_style(self) -> Style {
        Style::new()
    }
}

impl Display for Reset {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", Style::new())
    }
}

impl PartialEq<Style> for Reset {
    fn eq(&self, other: &Style) -> bool {
        self.to_style() == *other
    }
}
