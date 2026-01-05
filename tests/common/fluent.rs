macro_rules! test_fluent_type {
    ( $( $mod:ident { $value:expr, $as_style:expr } ),+ $(,)? ) => {
        mod fluent_type {
            $(
                mod $mod {
                    #[allow(unused_imports)]
                    use fluent_ansi::{*, color::*};

                    $crate::common::test_fluent_type!(NO_MOD: $value, $as_style);
                }
            )+
        }
    };

    ($value:expr, $as_style:expr) => {
        mod fluent_type {
            #[allow(unused_imports)]
            use fluent_ansi::{*, color::*};

            $crate::common::test_fluent_type!(NO_MOD: $value, $as_style);
        }
    };

    (NO_MOD: $value:expr, $as_style:expr) => {
        $crate::common::test_fluent_methods!(NO_MOD: $value, $as_style);

        #[test]
        fn applied_to() {
            let styled = $value.applied_to("CONTENT");

            assert_eq!(styled.get_content(), &"CONTENT");
            assert_eq!(styled.get_style(), $as_style);
        }

        #[test]
        fn to_style() {
            $crate::common::assert_from_to!(
                to_style, fluent_ansi::Style;
                $value,
                $as_style
            );
        }
    };
}
pub(crate) use test_fluent_type;

macro_rules! test_fluent_methods {
    ($value:expr, $as_composed_styling:expr) => {
        mod fluent_methods {
            use fluent_ansi::*;

            $crate::common::test_fluent_methods!(NO_MOD: $value, $as_composed_styling);
        }
    };

    (NO_MOD: $value:expr, $as_composed_styling:expr) => {
        #[test]
        fn effects() {
            use fluent_ansi::*;

            macro_rules! assert_effect_method {
                ($effect:expr, $method:ident) => {{
                    let expected_style = $as_composed_styling.$method();

                    assert_eq!(
                        $value.$method(),
                        expected_style,
                        "{:?}.{}()",
                        $value,
                        stringify!($method)
                    );
                    assert_eq!(
                        $value.effect($effect),
                        expected_style,
                        "{:?}.effect({:?})",
                        $value,
                        $effect
                    );
                    assert_eq!(
                        $value.add($effect),
                        expected_style,
                        "{:?}.add({:?})",
                        $value,
                        $effect
                    );
                }};
            }

            assert_effect_method!(Effect::Bold, bold);
            assert_effect_method!(Effect::Faint, faint);
            assert_effect_method!(Effect::Italic, italic);
            assert_effect_method!(Effect::SolidUnderline, solid_underline);
            assert_effect_method!(Effect::Blink, blink);
            assert_effect_method!(Effect::Reverse, reverse);
            assert_effect_method!(Effect::Conceal, conceal);
            assert_effect_method!(Effect::Strikethrough, strikethrough);
            assert_effect_method!(Effect::DoubleUnderline, double_underline);
            assert_effect_method!(Effect::Overline, overline);
            assert_effect_method!(Effect::UNDERLINE, underline);
        }

        #[test]
        fn underline_effects() {
            use fluent_ansi::*;

            macro_rules! assert_effect_method {
                ($underline_effect:expr, $method:ident) => {{
                    let expected_style = $as_composed_styling.$method();

                    assert_eq!(
                        $value.underline_effect($underline_effect),
                        expected_style,
                        "{:?}.underline_effect({:?})",
                        $value,
                        $underline_effect
                    );
                    assert_eq!(
                        $value.effect($underline_effect),
                        expected_style,
                        "{:?}.effect({:?})",
                        $value,
                        $underline_effect
                    );
                    assert_eq!(
                        $value.add($underline_effect),
                        expected_style,
                        "{:?}.add({:?})",
                        $value,
                        $underline_effect
                    );
                }};
            }

            assert_effect_method!(UnderlineEffect::Solid, solid_underline);
            assert_effect_method!(UnderlineEffect::Curly, curly_underline);
            assert_effect_method!(UnderlineEffect::Dotted, dotted_underline);
            assert_effect_method!(UnderlineEffect::Dashed, dashed_underline);
            assert_effect_method!(UnderlineEffect::Double, double_underline);
        }

        #[test]
        fn colors() {
            use fluent_ansi::prelude::*;

            macro_rules! assert_method_for_color {
                ($method:ident) => {
                    assert_method_for_color!($method, Color::RED);
                    assert_method_for_color!($method, Color::GREEN);
                };
                ($method:ident, $color:expr) => {{
                    let result = $value.$method($color);
                    let expected = $as_composed_styling.$method($color);
                    assert_eq!(result, expected);
                }};
            }

            macro_rules! assert_method_for_targeted_color {
                ($method:ident) => {
                    assert_method_for_targeted_color!($method, Color::RED);
                    assert_method_for_targeted_color!($method, Color::GREEN);
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
                    let expected = $as_composed_styling.$target_method($color);
                    assert_eq!(result, expected);
                }};
            }

            assert_method_for_color!(fg);
            assert_method_for_color!(bg);
            assert_method_for_color!(underline_color);

            assert_method_for_targeted_color!(color);
            assert_method_for_targeted_color!(add);
        }
    };
}
pub(crate) use test_fluent_methods;
