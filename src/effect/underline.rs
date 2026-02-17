use core::fmt::{Debug, Display, Formatter, Result};

use enum_iterator::Sequence;

use crate::{
    Effect,
    impl_macros::from_to::impl_from_to,
    traits::{Composed, StylingAttribute, StylingElement},
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

impl_from_to!(
    #[doc = r"Converts the type into an [`Effect`]."]
    fn to_effect(self: UnderlineEffect) -> Effect {
        match self {
            UnderlineEffect::Solid => Effect::SolidUnderline,
            UnderlineEffect::Curly => Effect::CurlyUnderline,
            UnderlineEffect::Dotted => Effect::DottedUnderline,
            UnderlineEffect::Dashed => Effect::DashedUnderline,
            UnderlineEffect::Double => Effect::DoubleUnderline,
        }
    }
);

impl Display for UnderlineEffect {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&self.to_effect(), f)
    }
}

impl StylingElement for UnderlineEffect {
    fn add_to<C: Composed>(self, composed: C) -> C {
        composed.set_underline_effect(Some(self))
    }
}

impl StylingAttribute for UnderlineEffect {
    type Value = bool;

    fn set_in<C: Composed>(self, composed: C, value: Self::Value) -> C {
        composed.set_effect(self.to_effect(), value)
    }

    fn get_from<C: Composed>(self, composed: &C) -> Self::Value {
        composed.get_effect(self.to_effect())
    }
}

/// The underline attribute.
///
/// Usable in the [`Composed::set`](crate::traits::Composed::set) and [`Composed::get`](crate::traits::Composed::get) methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnderlineStyle;

impl StylingAttribute for UnderlineStyle {
    type Value = Option<UnderlineEffect>;

    fn set_in<C: Composed>(self, composed: C, value: Self::Value) -> C {
        composed.set_underline_effect(value)
    }

    fn get_from<C: Composed>(self, composed: &C) -> Self::Value {
        composed.get_underline_effect()
    }
}
