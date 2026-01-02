use core::fmt::{Display, Formatter, Result};

use enum_iterator::Sequence;

use crate::{
    Effect, StyleAttribute, StyleElement, StyleSet,
    impl_macros::{fluent::impl_fluent_type, from_to::impl_from_to},
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

impl StyleElement for UnderlineStyle {
    fn add_to<S: StyleSet>(self, style_set: S) -> S {
        style_set.set_underline_style(Some(self))
    }
}

impl StyleAttribute for UnderlineStyle {
    type Value = bool;

    fn set_in<S: StyleSet>(self, style_set: S, value: Self::Value) -> S {
        style_set.set_effect(self.to_effect(), value)
    }

    fn get_from<S: StyleSet>(self, style_set: &S) -> Self::Value {
        style_set.get_effect(self.to_effect())
    }
}

/// The underline attribute.
///
/// Usable in the [`StyleSet::set`] and [`StyleSet::get`] methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Underline;

impl StyleAttribute for Underline {
    type Value = Option<UnderlineStyle>;

    fn set_in<S: StyleSet>(self, style_set: S, value: Self::Value) -> S {
        style_set.set_underline_style(value)
    }

    fn get_from<S: StyleSet>(self, style_set: &S) -> Self::Value {
        style_set.get_underline_style()
    }
}
