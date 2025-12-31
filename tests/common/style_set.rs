macro_rules! test_style_set {
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
                    let style_set = $empty_style_set.set(Underline, Some(UnderlineStyle::Solid));
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
pub(crate) use test_style_set;
