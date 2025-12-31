use crate::{Effect, StyleSet, TargetedColor, UnderlineStyle, colors::Color};

/// An element that can be added to a [`Style`](crate::Style).
///
/// This trait is used to define elements that can be added to a `Style`. Such elements
/// include effects ([`Effect`]) and colors (like [`TargetedColor`]).
pub trait StyleElement {
    /// Adds this element to the given `StyleSet`, returning it updated.
    #[must_use]
    fn add_to<S: StyleSet>(self, style_set: S) -> S;
}

/// A trait to set styling options on a type.
///
/// This trait is implemented by types that can be styled, such as [`Style`](crate::Style) and [`Styled`](crate::Styled).
/// It provides methods to set effects and colors, returning a type that implements [`StyleSet`].
pub trait ToStyleSet: Sized {
    /// The type that is returned by the styling methods.
    type StyleSet: StyleSet;

    /// Sets the bold effect.
    #[must_use]
    fn bold(self) -> Self::StyleSet {
        self.effect(Effect::Bold)
    }

    /// Sets the faint effect.
    #[must_use]
    fn faint(self) -> Self::StyleSet {
        self.effect(Effect::Faint)
    }

    /// Sets the italic effect.
    #[must_use]
    fn italic(self) -> Self::StyleSet {
        self.effect(Effect::Italic)
    }

    /// Sets the solid underline effect.
    #[must_use]
    fn underline(self) -> Self::StyleSet {
        self.effect(Effect::Underline)
    }

    /// Sets the curly underline effect.
    #[must_use]
    fn curly_underline(self) -> Self::StyleSet {
        self.effect(Effect::CurlyUnderline)
    }

    /// Sets the dotted underline effect.
    #[must_use]
    fn dotted_underline(self) -> Self::StyleSet {
        self.effect(Effect::DottedUnderline)
    }

    /// Sets the dashed underline effect.
    #[must_use]
    fn dashed_underline(self) -> Self::StyleSet {
        self.effect(Effect::DashedUnderline)
    }

    /// Sets the blink effect.
    #[must_use]
    fn blink(self) -> Self::StyleSet {
        self.effect(Effect::Blink)
    }

    /// Sets the reverse effect.
    #[must_use]
    fn reverse(self) -> Self::StyleSet {
        self.effect(Effect::Reverse)
    }

    /// Sets the conceal effect.
    #[must_use]
    fn conceal(self) -> Self::StyleSet {
        self.effect(Effect::Conceal)
    }

    /// Sets the strikethrough effect.
    #[must_use]
    fn strikethrough(self) -> Self::StyleSet {
        self.effect(Effect::Strikethrough)
    }

    /// Sets the double underline effect.
    #[must_use]
    fn double_underline(self) -> Self::StyleSet {
        self.effect(Effect::DoubleUnderline)
    }

    /// Sets the overline effect.
    #[must_use]
    fn overline(self) -> Self::StyleSet {
        self.effect(Effect::Overline)
    }

    /// Sets the given effect.
    #[must_use]
    fn effect(self, effect: impl Into<Effect>) -> Self::StyleSet {
        self.to_style_set().set_effect(effect, true)
    }

    /// Sets the underline style.
    #[must_use]
    fn underline_style(self, underline_style: UnderlineStyle) -> Self::StyleSet {
        self.effect(underline_style)
    }

    /// Sets the foreground color.
    #[must_use]
    fn fg(self, color: impl Into<Color>) -> Self::StyleSet {
        self.color(TargetedColor::new_for_fg(color))
    }

    /// Sets the background color.
    #[must_use]
    fn bg(self, color: impl Into<Color>) -> Self::StyleSet {
        self.color(TargetedColor::new_for_bg(color))
    }

    /// Sets the underline color.
    #[must_use]
    fn underline_color(self, color: impl Into<Color>) -> Self::StyleSet {
        self.color(TargetedColor::new_for_underline(color))
    }

    /// Sets the given color in a target.
    #[must_use]
    fn color(self, targeted_color: impl Into<TargetedColor>) -> Self::StyleSet {
        let targeted_color = targeted_color.into();
        self.to_style_set().set_color(
            targeted_color.get_target(),
            Some(targeted_color.get_color()),
        )
    }

    /// Adds the given element to the style.
    #[must_use]
    fn add(self, element: impl StyleElement) -> Self::StyleSet {
        let style_set = self.to_style_set();
        element.add_to(style_set)
    }

    /// Converts this value to a style set.
    #[must_use]
    fn to_style_set(self) -> Self::StyleSet;
}

#[cfg(test)]
pub(crate) mod tests {
    /// Includes tests for the [`ToStyleSet`] trait methods.
    ///
    /// # Arguments
    ///
    /// - `$value` is the value that is having its methods from [`ToStyleSet`] tested.
    /// - `$style_set` is the [`StyleSet`] value that corresponds to the `$value`.
    macro_rules! test_to_style_set_methods {
        ($mod:ident; $value:expr, $style_set:expr) => {
            mod $mod {
                $crate::to_style_set::tests::test_to_style_set_methods!($value, $style_set);
            }
        };
        ($value:expr, $style_set:expr) => {
            mod to_style_set {
                use crate::{color::*, *};

                #[test]
                fn effects() {
                    let value = $value;

                    macro_rules! assert_effect_method {
                        ($effect:expr, $method:ident) => {{
                            let expected_style = $style_set.$method();

                            assert_eq!(
                                value.$method(),
                                expected_style,
                                "{}.{}()",
                                stringify!($value),
                                stringify!($method)
                            );
                            assert_eq!(
                                value.effect($effect),
                                expected_style,
                                "{}.effect({})",
                                stringify!($value),
                                stringify!($effect)
                            );
                            assert_eq!(
                                value.add($effect),
                                expected_style,
                                "{}.add({})",
                                stringify!($value),
                                stringify!($effect)
                            );
                        }};
                    }

                    assert_effect_method!(Effect::Bold, bold);
                    assert_effect_method!(Effect::Faint, faint);
                    assert_effect_method!(Effect::Italic, italic);
                    assert_effect_method!(Effect::Underline, underline);
                    assert_effect_method!(Effect::Blink, blink);
                    assert_effect_method!(Effect::Reverse, reverse);
                    assert_effect_method!(Effect::Conceal, conceal);
                    assert_effect_method!(Effect::Strikethrough, strikethrough);
                    assert_effect_method!(Effect::DoubleUnderline, double_underline);
                    assert_effect_method!(Effect::Overline, overline);
                }

                #[test]
                fn underline_styles() {
                    let value = $value;

                    macro_rules! assert_effect_method {
                        ($underline_style:expr, $method:ident) => {{
                            let expected_style = $style_set.$method();

                            assert_eq!(
                                value.underline_style($underline_style),
                                expected_style,
                                "{}.effect({})",
                                stringify!($value),
                                stringify!($effect)
                            );
                            assert_eq!(
                                value.effect($underline_style),
                                expected_style,
                                "{}.effect({})",
                                stringify!($value),
                                stringify!($effect)
                            );
                            assert_eq!(
                                value.add($underline_style),
                                expected_style,
                                "{}.add({})",
                                stringify!($value),
                                stringify!($effect)
                            );
                        }};
                    }

                    assert_effect_method!(UnderlineStyle::Solid, underline);
                    assert_effect_method!(UnderlineStyle::Curly, curly_underline);
                    assert_effect_method!(UnderlineStyle::Dotted, dotted_underline);
                    assert_effect_method!(UnderlineStyle::Dashed, dashed_underline);
                    assert_effect_method!(UnderlineStyle::Double, double_underline);
                }

                #[test]
                fn colors() {
                    macro_rules! assert_method_for_color {
                        ($method:ident) => {
                            assert_method_for_color!($method, BasicColor::Red);
                            assert_method_for_color!($method, BasicColor::Green);
                        };
                        ($method:ident, $color:expr) => {{
                            let result = $value.$method($color);
                            let expected = $style_set.$method($color);
                            assert_eq!(result, expected);
                        }};
                    }

                    macro_rules! assert_method_for_targeted_color {
                        ($method:ident) => {
                            assert_method_for_targeted_color!($method, BasicColor::Red);
                            assert_method_for_targeted_color!($method, BasicColor::Green);
                        };
                        ($method:ident, $color:expr) => {
                            // Foreground by default
                            assert_method_for_targeted_color!($method, $color, fg, $color);

                            // With explicit color target
                            assert_method_for_targeted_color!($method, $color, fg, $color.for_fg());
                            assert_method_for_targeted_color!($method, $color, bg, $color.for_bg());
                            assert_method_for_targeted_color!(
                                $method,
                                $color,
                                underline_color,
                                $color.for_underline()
                            );
                        };
                        ($method:ident, $color:expr, $target_method:ident, $arg:expr) => {{
                            let result = $value.$method($arg);
                            let expected = $style_set.$target_method($color);
                            assert_eq!(result, expected);
                        }};
                    }

                    assert_method_for_color!(fg);
                    assert_method_for_color!(bg);
                    assert_method_for_color!(underline_color);

                    assert_method_for_targeted_color!(color);
                    assert_method_for_targeted_color!(add);
                }

                #[test]
                fn to_style_set() {
                    assert_eq!(
                        $value.to_style_set(),
                        $style_set,
                        "{}.to_style_set()",
                        stringify!($value)
                    );
                }
            }
        };
    }
    pub(crate) use test_to_style_set_methods;
}
