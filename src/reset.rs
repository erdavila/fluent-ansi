use core::fmt::{Display, Formatter, Result};

use crate::{Format, ToFormat};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Reset;

impl Display for Reset {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", Format::new())
    }
}

impl PartialEq<Format> for Reset {
    fn eq(&self, other: &Format) -> bool {
        self.to_format() == *other
    }
}

impl ToFormat for Reset {
    fn to_format(self) -> Format {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{ToFormatSet as _, assert_display};

    use super::*;

    #[test]
    fn reset() {
        assert_display!(Reset, "\x1b[0m");
    }

    #[test]
    fn eq() {
        assert_eq!(Reset, Reset);
        assert_eq!(Reset, Format::new());
        assert_ne!(Reset, Format::new().bold());
        assert_eq!(Format::new(), Reset);
        assert_ne!(Format::new().bold(), Reset);
    }

    #[test]
    fn to_format() {
        assert_eq!(Reset.to_format(), Format::new());
    }
}
