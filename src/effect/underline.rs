use core::fmt::{Debug, Display, Formatter, Result};

use enum_iterator::Sequence;

use crate::{
    Effect, Style,
    macros::{impl_add_for_additive_type, impl_add_for_to_style_type},
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

    /// Converts the type into an [`Effect`].
    #[must_use]
    pub fn to_effect(self: UnderlineEffect) -> Effect {
        self.into()
    }
}

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

impl_add_for_additive_type!(for UnderlineEffect, Output = Style);

impl_add_for_to_style_type!(for UnderlineEffect);

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
