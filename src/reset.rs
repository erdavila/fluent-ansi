use core::fmt::{Display, Formatter, Result};

use crate::Style;

/// A type that represents the reset of all styling.
///
/// When rendered, it produces the ANSI escape sequence to reset all styling.
///
/// See [The `Reset` singleton](crate#the-reset-singleton).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Reset;

impl From<Reset> for Style {
    fn from(_: Reset) -> Self {
        Style::new()
    }
}

impl Display for Reset {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", Style::new())
    }
}
