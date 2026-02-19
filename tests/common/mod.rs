#![allow(unused)]

mod additive_styling;
mod color_kind;
mod composed_styling;
mod from_to;

pub(crate) use additive_styling::*;
pub(crate) use color_kind::*;
pub(crate) use composed_styling::*;
pub(crate) use from_to::*;

/// A macro to assert that a type implementing `Display` produces the expected output.
macro_rules! assert_display {
    ($display:expr, $expected:literal) => {{
        use core::fmt::Write as _;
        let mut s = String::new();

        write!(&mut s, "{}", $display).unwrap();

        assert_eq!(s.as_str(), $expected);
    }};
}
pub(crate) use assert_display;
