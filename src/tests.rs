#[macro_export]
macro_rules! assert_display {
    ($display:expr, $expected:literal) => {{
        use core::fmt::Write as _;
        let mut vec = arrayvec::ArrayString::<30>::new();

        write!(&mut vec, "{}", $display).unwrap();

        assert_eq!(vec.as_str(), $expected);
    }};
}
