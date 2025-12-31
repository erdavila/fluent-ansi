macro_rules! test_to_style_set {
    ( $( $mod:ident { $( $tt:tt )+ } ),+ $(,)? ) => {
        mod to_style_set {
            $(
                mod $mod {
                    $crate::common::test_to_style_set!(NO_MOD: $( $tt )+ );
                }
            )+
        }
    };

    ($value:expr, $as_style_set:expr) => {
        mod to_style_set {
            $crate::common::test_to_style_set!(NO_MOD: $value, $as_style_set);
        }
    };

    (NO_MOD: $value:expr, $as_style_set:expr) => {
        use fluent_ansi::{color::*, prelude::*, *};

        #[test]
        fn effects() {
            let value = $value;

            macro_rules! assert_effect_method {
                ($effect:expr, $method:ident) => {{
                    let expected_style = $as_style_set.$method();

                    assert_eq!(
                        value.$method(),
                        expected_style,
                        "{:?}.{}()",
                        $value,
                        stringify!($method)
                    );
                    assert_eq!(
                        value.effect($effect),
                        expected_style,
                        "{:?}.effect({:?})",
                        $value,
                        $effect
                    );
                    assert_eq!(
                        value.add($effect),
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
                    let expected_style = $as_style_set.$method();

                    assert_eq!(
                        value.underline_style($underline_style),
                        expected_style,
                        "{:?}.underline_style({:?})",
                        $value,
                        $underline_style
                    );
                    assert_eq!(
                        value.effect($underline_style),
                        expected_style,
                        "{:?}.effect({:?})",
                        $value,
                        $underline_style
                    );
                    assert_eq!(
                        value.add($underline_style),
                        expected_style,
                        "{:?}.add({:?})",
                        $value,
                        $underline_style
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
                    let expected = $as_style_set.$method($color);
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
                    let expected = $as_style_set.$target_method($color);
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
                $as_style_set,
                "{:?}.to_style_set()",
                $value
            );
        }
    };
}
pub(crate) use test_to_style_set;
