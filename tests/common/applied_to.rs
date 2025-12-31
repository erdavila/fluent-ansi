macro_rules! test_applied_to {
    ($value:expr, $expected_style:expr) => {
        #[test]
        fn applied_to() {
            let styled = $value.applied_to("CONTENT");

            assert_eq!(styled.get_content(), &"CONTENT");
            assert_eq!(styled.get_style(), $expected_style);
        }
    };
}
pub(crate) use test_applied_to;
