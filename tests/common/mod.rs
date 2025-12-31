#![allow(unused)]

mod applied_to;
mod color_type;
mod from_to;
mod style_set;
mod to_style_set;

pub(crate) use applied_to::*;
pub(crate) use color_type::*;
pub(crate) use from_to::*;
pub(crate) use style_set::*;
pub(crate) use to_style_set::*;

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
