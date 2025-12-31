macro_rules! test_color_type {
    ( $( $mod:ident { $( $tt:tt )+ } ),+ $(,)? ) => {
        mod color_type {
            $(
                mod $mod {
                    $crate::common::test_color_type!(NO_MOD: $( $tt )+ );
                }
            )+
        }
    };

    ($color:expr, $as_color:expr, $as_style:expr) => {
        mod color_type {
            $crate::common::test_color_type!(NO_MOD: $color, $as_color, $as_style);
        }
    };

    (NO_MOD: $color:expr, $as_color:expr, $as_style:expr) => {
        use fluent_ansi::{color::*, prelude::*, *};

        #[test]
        fn for_fg() {
            assert_eq!(
                $color.for_fg(),
                TargetedColor::new($color, ColorTarget::Foreground)
            );
            assert_eq!(
                $color.for_target(ColorTarget::Foreground),
                TargetedColor::new($color, ColorTarget::Foreground)
            );
        }

        #[test]
        fn for_bg() {
            assert_eq!(
                $color.for_bg(),
                TargetedColor::new($color, ColorTarget::Background)
            );
            assert_eq!(
                $color.for_target(ColorTarget::Background),
                TargetedColor::new($color, ColorTarget::Background)
            );
        }

        #[test]
        fn for_underline() {
            assert_eq!(
                $color.for_underline(),
                TargetedColor::new($color, ColorTarget::Underline)
            );
            assert_eq!(
                $color.for_target(ColorTarget::Underline),
                TargetedColor::new($color, ColorTarget::Underline)
            );
        }

        $crate::common::test_applied_to!($color, Style::new().fg($color));

        #[test]
        fn to_color() {
            assert_eq!($color.to_color(), $as_color);
            assert_eq!(Color::from($color), $as_color);
        }

        #[test]
        fn to_style() {
            assert_eq!($color.to_style(), $as_style);
            assert_eq!(Style::from($color), $as_style);
        }
    };
}
pub(crate) use test_color_type;

macro_rules! test_to_style_set_with_fg_assumed {
    ( $( $mod:ident { $color:expr } ),+ $(,)? ) => {
        mod to_style_set_with_fg_assumed {
            $(
                mod $mod {
                    $crate::common::test_to_style_set!(NO_MOD: $color , Style::new().fg($color));
                }
            )+
        }
    };

    ($color:expr) => {
        mod to_style_set_with_fg_assumed {
            $crate::common::test_to_style_set!(NO_MOD: $color, Style::new().fg($color));
        }
    };
}
pub(crate) use test_to_style_set_with_fg_assumed;
