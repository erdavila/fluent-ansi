use core::fmt::{Display, Formatter, Result};

use enum_iterator::Sequence;

use crate::{
    AppliedTo, Effect, Style, StyleAttribute, StyleElement, StyleSet, ToStyle, ToStyleSet,
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

    #[must_use]
    pub(crate) fn to_effect(self) -> Effect {
        self.into()
    }
}

impl AppliedTo for UnderlineStyle {}

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

impl ToStyle for UnderlineStyle {
    fn to_style(self) -> Style {
        self.into()
    }
}

impl StyleElement for UnderlineStyle {
    fn add_to_style(self, style: Style) -> Style {
        style.set_underline_style(Some(self))
    }
}

impl StyleAttribute for UnderlineStyle {
    type Value = bool;

    fn set_in_style(self, style: Style, value: Self::Value) -> Style {
        style.set_effect(self.to_effect(), value)
    }

    fn get_from_style(self, style: &Style) -> Self::Value {
        style.get_effect(self.to_effect())
    }
}

/// The underline attribute.
///
/// Usable in the [`StyleSet::set`] and [`StyleSet::get`] methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Underline;

impl StyleAttribute for Underline {
    type Value = Option<UnderlineStyle>;

    fn set_in_style(self, style: Style, value: Self::Value) -> Style {
        let encoded_effects = style.encoded_effects.set_underline(value);
        Style {
            encoded_effects,
            ..style
        }
    }

    fn get_from_style(self, style: &Style) -> Self::Value {
        UnderlineStyle::all()
            .find(|&underline_style| style.encoded_effects.get(underline_style.to_effect()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{tests::assert_display, to_style_set::tests::test_to_style_set_methods};

    use super::*;

    test_to_style_set_methods!(solid; UnderlineStyle::Solid, Style::new().underline());
    test_to_style_set_methods!(curly; UnderlineStyle::Curly, Style::new().curly_underline());

    #[test]
    fn applied_to() {
        let stld = UnderlineStyle::Curly.applied_to("CONTENT");

        assert_eq!(stld.get_content(), &"CONTENT");
        assert_eq!(stld.get_style(), Style::new().curly_underline());
    }

    #[test]
    fn to_effect() {
        assert_eq!(UnderlineStyle::Solid.to_effect(), Effect::Underline);
        assert_eq!(UnderlineStyle::Curly.to_effect(), Effect::CurlyUnderline);
        assert_eq!(UnderlineStyle::Dotted.to_effect(), Effect::DottedUnderline);
        assert_eq!(UnderlineStyle::Dashed.to_effect(), Effect::DashedUnderline);
        assert_eq!(UnderlineStyle::Double.to_effect(), Effect::DoubleUnderline);
    }

    #[test]
    fn to_style() {
        assert_eq!(UnderlineStyle::Solid.to_style(), Style::new().underline());
        assert_eq!(
            UnderlineStyle::Curly.to_style(),
            Style::new().curly_underline()
        );
        assert_eq!(
            UnderlineStyle::Dotted.to_style(),
            Style::new().dotted_underline()
        );
        assert_eq!(
            UnderlineStyle::Dashed.to_style(),
            Style::new().dashed_underline()
        );
        assert_eq!(
            UnderlineStyle::Double.to_style(),
            Style::new().double_underline()
        );
    }

    #[test]
    fn display() {
        assert_display!(UnderlineStyle::Solid, "\x1b[4m");
        assert_display!(UnderlineStyle::Curly, "\x1b[4:3m");
        assert_display!(UnderlineStyle::Dotted, "\x1b[4:4m");
        assert_display!(UnderlineStyle::Dashed, "\x1b[4:5m");
        assert_display!(UnderlineStyle::Double, "\x1b[21m");
    }
}
