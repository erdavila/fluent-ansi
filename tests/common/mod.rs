#![allow(unused)]

mod additive;
mod color_kind;
mod composed;
mod from_to;

pub(crate) use additive::*;
pub(crate) use color_kind::*;
pub(crate) use composed::*;
pub(crate) use from_to::*;

/// A macro to assert that a type implementing `Display` produces the expected output.
macro_rules! assert_display {
    ($display:expr, $expected:literal) => {{
        // Every type that implements `Display` also implements `ToString`, so we can call `to_string()` on it.
        assert_eq!($display.to_string(), $expected);
    }};
}
pub(crate) use assert_display;
