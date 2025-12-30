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
