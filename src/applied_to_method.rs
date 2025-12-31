// Dependency: for the current type T, impl From<T> for Style.
macro_rules! applied_to_method {
    () => {
        /// Applies the styling to the given content, returning a [`Styled<C>`](crate::Styled) instance.
        #[must_use]
        pub fn applied_to<C: core::fmt::Display>(self, content: C) -> $crate::Styled<C> {
            let style = $crate::Style::from(self);
            $crate::Styled::new(content).with_style(style)
        }
    };
}
pub(crate) use applied_to_method;

#[cfg(test)]
pub(crate) mod tests {
    macro_rules! test_applied_to_method {
        ($value:expr, $expected_style:expr) => {
            #[test]
            fn applied_to() {
                let styled = $value.applied_to("CONTENT");

                assert_eq!(styled.get_content(), &"CONTENT");
                assert_eq!(styled.get_style(), $expected_style);
            }
        };
    }
    pub(crate) use test_applied_to_method;
}
