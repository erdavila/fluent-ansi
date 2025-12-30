use crate::{
    ColorTarget, Effect, GetEffects, Style, ToStyleSet, Underline, UnderlineStyle, color::Color,
};

/// A trait to represent an attribute that can be set or retrieved from a [`Style`].
pub trait StyleAttribute {
    /// The type of value associated with this attribute.
    type Value: Default;

    /// Sets this attribute in the given `Style`, returning the updated `Style`.
    #[must_use]
    fn set_in_style(self, style: Style, value: Self::Value) -> Style;

    /// Gets the value of this attribute from the given `Style`.
    #[must_use]
    fn get_from_style(self, style: &Style) -> Self::Value;
}

/// A trait to set and get styling options on a type.
///
/// This trait extends [`ToStyleSet`] with methods to get the current state of styling options,
/// such as checking if an effect is set or getting the color of a target.
pub trait StyleSet: ToStyleSet<StyleSet = Self> {
    /// Sets the given effect to the specified value.
    #[must_use]
    fn set_effect(self, effect: impl Into<Effect>, value: bool) -> Self {
        self.set(effect.into(), value)
    }

    /// Gets whether the given effect is set.
    #[must_use]
    fn get_effect(&self, effect: impl Into<Effect>) -> bool {
        self.get(effect.into())
    }

    /// Returns an iterator over the effects that are currently set.
    #[must_use]
    fn get_effects(&self) -> GetEffects;

    /// Sets the underline style.
    #[must_use]
    fn set_underline_style(self, underline_style: Option<UnderlineStyle>) -> Self {
        self.set(Underline, underline_style)
    }

    /// Gets the underline style.
    #[must_use]
    fn get_underline_style(&self) -> Option<UnderlineStyle> {
        UnderlineStyle::all().find(|&underline_style| self.get_effect(underline_style.to_effect()))
    }

    /// Sets the color for the given color target.
    ///
    /// Use [`Color::none()`] to clear the color for some color target:
    ///
    /// ```
    /// # use fluent_ansi::{prelude::*, ColorTarget, Style};
    /// # let style_set = Style::new();
    /// style_set.set_color(ColorTarget::Foreground, Color::none());
    /// ```
    #[must_use]
    fn set_color(self, target: ColorTarget, color: Option<impl Into<Color>>) -> Self {
        let color: Option<Color> = color.map(Into::into);
        self.set(target, color)
    }

    /// Gets the color for the given color target.
    #[must_use]
    fn get_color(&self, target: ColorTarget) -> Option<Color> {
        self.get(target)
    }

    /// Sets the given attribute to the specified value.
    #[must_use]
    fn set<A: StyleAttribute>(self, attr: A, value: A::Value) -> Self;

    /// Gets the value of the given attribute.
    #[must_use]
    fn get<A: StyleAttribute>(&self, attr: A) -> A::Value;

    /// Unsets the given attribute.
    #[must_use]
    fn unset<A: StyleAttribute>(self, attr: A) -> Self {
        self.set(attr, A::Value::default())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    /// Includes tests for the [`StyleSet`](crate::StyleSet) trait methods.
    macro_rules! test_style_set_methods {
        ($empty_style_set:expr) => {
            mod style_set {
                use crate::{color::*, *};

                #[test]
                fn effects() {
                    let style_set = $empty_style_set;
                    assert_eq!(style_set.get_effect(Effect::Bold), false);
                    assert_eq!(style_set.get(Effect::Bold), false);
                    assert_eq!(style_set.get_effect(Effect::Italic), false);
                    assert_eq!(style_set.get(Effect::Italic), false);

                    {
                        let style_set = $empty_style_set.set_effect(Effect::Bold, true);
                        assert_eq!(style_set, $empty_style_set.bold());
                        assert_eq!(style_set.get_effect(Effect::Bold), true);
                        assert_eq!(style_set.get(Effect::Bold), true);
                        assert_eq!(style_set.get_effect(Effect::Italic), false);
                        assert_eq!(style_set.get(Effect::Italic), false);

                        let style_set = style_set.set_effect(Effect::Bold, false);
                        assert_eq!(style_set, $empty_style_set);
                        assert_eq!(style_set.get_effect(Effect::Bold), false);
                        assert_eq!(style_set.get(Effect::Bold), false);
                    }

                    {
                        let style_set = $empty_style_set.set(Effect::Bold, true);
                        assert_eq!(style_set, $empty_style_set.bold());
                        assert_eq!(style_set.get_effect(Effect::Bold), true);
                        assert_eq!(style_set.get(Effect::Bold), true);

                        let style_set = style_set.unset(Effect::Bold);
                        assert_eq!(style_set, $empty_style_set);
                        assert_eq!(style_set.get_effect(Effect::Bold), false);
                        assert_eq!(style_set.get(Effect::Bold), false);
                    }
                }

                #[test]
                fn get_effects() {
                    let style_set = $empty_style_set.bold().italic().underline();
                    let mut effects = style_set.get_effects();

                    assert_eq!(effects.next(), Some(Effect::Bold));
                    assert_eq!(effects.next(), Some(Effect::Italic));
                    assert_eq!(effects.next(), Some(Effect::Underline));
                    assert_eq!(effects.next(), None);
                }

                #[test]
                fn underline_styles() {
                    let style_set = $empty_style_set;
                    assert_eq!(style_set.get_underline_style(), None);
                    assert_eq!(style_set.get(Underline), None);

                    {
                        let style_set =
                            $empty_style_set.set_underline_style(Some(UnderlineStyle::Solid));
                        assert_eq!(style_set, $empty_style_set.underline());
                        assert_eq!(style_set.get_underline_style(), Some(UnderlineStyle::Solid));
                        assert_eq!(style_set.get(Underline), Some(UnderlineStyle::Solid));

                        let style_set = style_set.set_underline_style(None);
                        assert_eq!(style_set, $empty_style_set);
                        assert_eq!(style_set.get_underline_style(), None);
                        assert_eq!(style_set.get(Underline), None);
                    }

                    {
                        let style_set =
                            $empty_style_set.set(Underline, Some(UnderlineStyle::Solid));
                        assert_eq!(style_set, $empty_style_set.underline());
                        assert_eq!(style_set.get_underline_style(), Some(UnderlineStyle::Solid));
                        assert_eq!(style_set.get(Underline), Some(UnderlineStyle::Solid));

                        let style_set = style_set.unset(Underline);
                        assert_eq!(style_set, $empty_style_set);
                        assert_eq!(style_set.get_underline_style(), None);
                        assert_eq!(style_set.get(Underline), None);
                    }

                    {
                        let style_set = $empty_style_set.set(UnderlineStyle::Solid, true);
                        assert_eq!(style_set, $empty_style_set.underline());
                        assert_eq!(style_set.get_underline_style(), Some(UnderlineStyle::Solid));
                        assert_eq!(style_set.get(Underline), Some(UnderlineStyle::Solid));
                        assert_eq!(style_set.get(UnderlineStyle::Solid), true);

                        let style_set = style_set.unset(UnderlineStyle::Solid);
                        assert_eq!(style_set, $empty_style_set);
                        assert_eq!(style_set.get_underline_style(), None);
                        assert_eq!(style_set.get(Underline), None);
                        assert_eq!(style_set.get(UnderlineStyle::Solid), false);
                    }

                    {
                        let style_set = $empty_style_set.set(UnderlineStyle::Solid, true);
                        assert_eq!(style_set, $empty_style_set.underline());
                        assert_eq!(style_set.get_underline_style(), Some(UnderlineStyle::Solid));
                        assert_eq!(style_set.get(Underline), Some(UnderlineStyle::Solid));
                        assert_eq!(style_set.get(UnderlineStyle::Solid), true);

                        let style_set = style_set.set(UnderlineStyle::Solid, false);
                        assert_eq!(style_set, $empty_style_set);
                        assert_eq!(style_set.get_underline_style(), None);
                        assert_eq!(style_set.get(Underline), None);
                        assert_eq!(style_set.get(UnderlineStyle::Solid), false);
                    }

                    {
                        let style_set = $empty_style_set.set_effect(UnderlineStyle::Solid, true);
                        assert_eq!(style_set, $empty_style_set.underline());
                        assert_eq!(style_set.get_underline_style(), Some(UnderlineStyle::Solid));
                        assert_eq!(style_set.get(Underline), Some(UnderlineStyle::Solid));
                        assert_eq!(style_set.get_effect(UnderlineStyle::Solid), true);

                        let style_set = style_set.set_effect(UnderlineStyle::Solid, false);
                        assert_eq!(style_set, $empty_style_set);
                        assert_eq!(style_set.get_underline_style(), None);
                        assert_eq!(style_set.get(Underline), None);
                        assert_eq!(style_set.get_effect(UnderlineStyle::Solid), false);
                    }
                }

                macro_rules! assert_targeted_color {
                    ($color_target:expr, $method:ident) => {
                        let empty_style_set = $empty_style_set;
                        assert_eq!(empty_style_set.get_color($color_target), None);
                        assert_eq!(empty_style_set.get($color_target), None);

                        let style_set =
                            $empty_style_set.set_color($color_target, Some(BasicColor::Red));
                        assert_eq!(style_set, $empty_style_set.$method(BasicColor::Red));
                        assert_eq!(
                            style_set.get_color($color_target),
                            Some(BasicColor::Red.to_color())
                        );
                        assert_eq!(
                            style_set.get($color_target),
                            Some(BasicColor::Red.to_color())
                        );

                        let style_set =
                            $empty_style_set.set($color_target, Some(BasicColor::Red.to_color()));
                        assert_eq!(style_set, $empty_style_set.$method(BasicColor::Red));
                        assert_eq!(
                            style_set.get_color($color_target),
                            Some(BasicColor::Red.to_color())
                        );
                        assert_eq!(
                            style_set.get($color_target),
                            Some(BasicColor::Red.to_color())
                        );

                        let style_set =
                            $empty_style_set.set_color($color_target, Some(BasicColor::Red));

                        {
                            let empty_style_set = style_set.set_color($color_target, None::<Color>);
                            assert_eq!(empty_style_set, $empty_style_set);
                            assert_eq!(empty_style_set.get_color($color_target), None);
                            assert_eq!(empty_style_set.get($color_target), None);
                        }

                        {
                            let empty_style_set = style_set.unset($color_target);
                            assert_eq!(empty_style_set, $empty_style_set);
                            assert_eq!(empty_style_set.get_color($color_target), None);
                            assert_eq!(empty_style_set.get($color_target), None);
                        }
                    };
                }

                #[test]
                fn foreground_color() {
                    assert_targeted_color!(ColorTarget::Foreground, fg);
                }

                #[test]
                fn background_color() {
                    assert_targeted_color!(ColorTarget::Background, bg);
                }

                #[test]
                fn underline_color() {
                    assert_targeted_color!(ColorTarget::Underline, underline_color);
                }
            }
        };
    }
    pub(crate) use test_style_set_methods;
}
