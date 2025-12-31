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

#[cfg(test)]
mod tests {
    use crate::{ToStyleSet as _, tests::assert_display};

    use super::*;

    #[test]
    fn reset() {
        assert_display!(Reset, "\x1b[0m");
    }

    #[test]
    fn eq() {
        assert_eq!(Reset, Reset);
        assert_eq!(Reset, Style::new());
        assert_ne!(Reset, Style::new().bold());
        assert_eq!(Style::new(), Reset);
        assert_ne!(Style::new().bold(), Reset);
    }

    #[test]
    fn to_style() {
        assert_eq!(Reset.to_style(), Style::new());
    }
}
