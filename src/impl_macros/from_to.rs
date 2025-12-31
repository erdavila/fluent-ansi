macro_rules! impl_from_to {
    (#[doc = $doc:literal] fn $to_method:ident ( $self:ident : $from:ty ) -> $to:ty $impl:block) => {
        impl $from {
            #[doc = $doc]
            #[must_use]
            pub fn $to_method($self) -> $to {
                $impl
            }
        }

        impl From<$from> for $to {
            fn from(value: $from) -> Self {
                value.$to_method()
            }
        }
    };
}
pub(crate) use impl_from_to;
