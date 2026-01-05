use core::fmt::{Display, Formatter, Result};

use enum_iterator::Sequence;

use crate::{
    Effect,
    impl_macros::{fluent::impl_fluent_type, from_to::impl_from_to},
    impl_styling_atribute_for, impl_styling_element_for,
};

pub(crate) type AllUnderlineEffects = enum_iterator::All<UnderlineEffect>;

/// An enumeration of all supported underline effects.
///
/// The values correspond to a subset of [`Effect`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Sequence)]
pub enum UnderlineEffect {
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

impl UnderlineEffect {
    #[must_use]
    pub(crate) fn all() -> AllUnderlineEffects {
        enum_iterator::all()
    }
}

impl_fluent_type!(UnderlineEffect {
    args: [self];
    to_style: { self.to_effect().to_style() }
});

impl_from_to!(
    #[doc = r"Converts the type into an [`Effect`]."]
    fn to_effect(self: UnderlineEffect) -> Effect {
        match self {
            UnderlineEffect::Solid => Effect::Underline,
            UnderlineEffect::Curly => Effect::CurlyUnderline,
            UnderlineEffect::Dotted => Effect::DottedUnderline,
            UnderlineEffect::Dashed => Effect::DashedUnderline,
            UnderlineEffect::Double => Effect::DoubleUnderline,
        }
    }
);

impl Display for UnderlineEffect {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.to_effect().fmt(f)
    }
}

impl_styling_element_for! { UnderlineEffect {
    args: [self, composed_styling];
    add_to: {
        composed_styling.set_underline_effect(Some(self))
    }
}}

impl_styling_atribute_for! { UnderlineEffect {
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
pub struct UnderlineStyle;

impl_styling_atribute_for! { UnderlineStyle {
    type Value = Option<UnderlineEffect>;
    args: [self, composed_styling, value];

    set_in: {
        composed_styling.set_underline_effect(value)
    }

    get_from: {
        composed_styling.get_underline_effect()
    }
}}
