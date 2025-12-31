use core::fmt::{Display, Formatter, Result};

use enum_iterator::Sequence;

use crate::{
    Effect, Style, StyleAttribute, StyleElement, StyleSet, ToStyleSet,
    impl_macros::applied_to::impl_applied_to,
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
    impl_applied_to!();

    /// Converts the type into a [`Style`].
    #[must_use]
    pub fn to_style(self) -> Style {
        Style::new().underline_style(self)
    }

    #[must_use]
    pub(crate) fn all() -> AllUnderlineStyles {
        enum_iterator::all()
    }

    /// Converts the type into an [`Effect`].
    #[must_use]
    pub fn to_effect(self) -> Effect {
        match self {
            UnderlineStyle::Solid => Effect::Underline,
            UnderlineStyle::Curly => Effect::CurlyUnderline,
            UnderlineStyle::Dotted => Effect::DottedUnderline,
            UnderlineStyle::Dashed => Effect::DashedUnderline,
            UnderlineStyle::Double => Effect::DoubleUnderline,
        }
    }
}

impl Display for UnderlineStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.to_effect().fmt(f)
    }
}

impl ToStyleSet for UnderlineStyle {
    type StyleSet = Style;

    fn to_style_set(self) -> Self::StyleSet {
        self.to_style()
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
