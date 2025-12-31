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
