use core::fmt::{Display, Formatter, Result};

use enum_iterator::Sequence;

use crate::{
    Effect,
    impl_macros::{fluent::impl_fluent_type, from_to::impl_from_to},
    impl_style_atribute_for, impl_style_element_for,
};

pub(crate) type AllUnderlineStyles = enum_iterator::All<UnderlineStyle>;

/// An enumeration of all supported underline styles.
///
/// The values correspond to a subset of [`Effect`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Sequence)]
pub enum UnderlineStyle {
    /// Solid underline styling.
    #[default]
    Solid,
    /// Curly underline styling.
    Curly,
    /// Dotted underline styling.
    Dotted,
    /// Dashed underline styling.
    Dashed,
    /// Double underline styling.
    Double,
}

impl UnderlineStyle {
    #[must_use]
    pub(crate) fn all() -> AllUnderlineStyles {
        enum_iterator::all()
    }
}

impl_fluent_type!(UnderlineStyle {
    args: [self];
    to_style: { self.to_effect().to_style() }
});

impl_from_to!(
    #[doc = r"Converts the type into an [`Effect`]."]
    fn to_effect(self: UnderlineStyle) -> Effect {
        match self {
            UnderlineStyle::Solid => Effect::Underline,
            UnderlineStyle::Curly => Effect::CurlyUnderline,
            UnderlineStyle::Dotted => Effect::DottedUnderline,
            UnderlineStyle::Dashed => Effect::DashedUnderline,
            UnderlineStyle::Double => Effect::DoubleUnderline,
        }
    }
);

impl Display for UnderlineStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.to_effect().fmt(f)
    }
}

impl_style_element_for! { UnderlineStyle {
    args: [self, composed_styling];
    add_to: {
        composed_styling.set_underline_style(Some(self))
    }
}}

impl_style_atribute_for! { UnderlineStyle {
    type Value = bool;
    args: [self, composed_styling, value];

    set_in: {
        composed_styling.set_effect(self.to_effect(), value)
    }

    get_from: {
        composed_styling.get_effect(self.to_effect())
    }
}}

/// The underline attribute.
///
/// Usable in the
/// [`Style::set`](crate::Style::set)/[`Styled::set`](crate::Styled::set) and
/// [`Style::get`](crate::Style::get)/[`Styled::get`](crate::Styled::get) methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Underline;

impl_style_atribute_for! { Underline {
    type Value = Option<UnderlineStyle>;
    args: [self, composed_styling, value];

    set_in: {
        composed_styling.set_underline_style(value)
    }

    get_from: {
        composed_styling.get_underline_style()
    }
}}
