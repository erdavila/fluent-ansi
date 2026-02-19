macro_rules! test_color_kind {
    ( $( $mod:ident { $( $tt:tt )+ } ),+ $(,)? ) => {
        mod color_kind {
            $(
                mod $mod {
                    $crate::common::test_color_kind!(NO_MOD: $( $tt )+ );
                }
            )+
        }
    };

    ($color:expr, $as_color:expr, $as_style:expr) => {
        mod color_kind {
            $crate::common::test_color_kind!(NO_MOD: $color, $as_color, $as_style);
        }
    };

    (NO_MOD: $color:expr, $as_color:expr, $as_style:expr) => {
        use fluent_ansi::{color::*, traits::ColorKind as _, *};
        use $crate::common::assert_from_to;

        #[test]
        fn for_foreground() {
            let expected = TargetedColor::new($color, ColorTarget::Foreground);

            assert_eq!($color.for_foreground(), expected);
            assert_eq!($color.for_fg(), expected);
            assert_eq!($color.for_target(ColorTarget::Foreground), expected);
            assert_eq!($color.for_target(ColorTarget::FG), expected);
        }

        #[test]
        fn for_background() {
            let expected = TargetedColor::new($color, ColorTarget::Background);

            assert_eq!($color.for_background(), expected);
            assert_eq!($color.for_bg(), expected);
            assert_eq!($color.for_target(ColorTarget::Background), expected);
            assert_eq!($color.for_target(ColorTarget::BG), expected);
        }

        #[test]
        fn for_underline() {
            let expected = TargetedColor::new($color, ColorTarget::Underline);

            assert_eq!($color.for_underline(), expected);
            assert_eq!($color.for_target(ColorTarget::Underline), expected);
        }

        $crate::common::test_additive_styling_type!(NO_MOD: $color, Style::new().fg($color));

        #[test]
        fn to_targeted_color() {
            assert_from_to!(
                to_targeted_color, TargetedColor;
                $color,
                TargetedColor::new($color, ColorTarget::Foreground)
            );
        }

        #[test]
        fn to_color() {
            assert_from_to!(
                to_color, Color;
                $color,
                $as_color
            );
        }
    };
}
pub(crate) use test_color_kind;
