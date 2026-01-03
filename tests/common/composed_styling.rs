macro_rules! test_composed_styling_type {
    ($empty_composed_styling:expr) => {
        mod composed_styling {
            use crate::{color::*, *};

            #[test]
            fn effects() {
                let composed_styling = $empty_composed_styling;
                assert_eq!(composed_styling.get_effect(Effect::Bold), false);
                assert_eq!(composed_styling.get(Effect::Bold), false);
                assert_eq!(composed_styling.get_effect(Effect::Italic), false);
                assert_eq!(composed_styling.get(Effect::Italic), false);

                {
                    let composed_styling = $empty_composed_styling.set_effect(Effect::Bold, true);
                    assert_eq!(composed_styling, $empty_composed_styling.bold());
                    assert_eq!(composed_styling.get_effect(Effect::Bold), true);
                    assert_eq!(composed_styling.get(Effect::Bold), true);
                    assert_eq!(composed_styling.get_effect(Effect::Italic), false);
                    assert_eq!(composed_styling.get(Effect::Italic), false);

                    let composed_styling = composed_styling.set_effect(Effect::Bold, false);
                    assert_eq!(composed_styling, $empty_composed_styling);
                    assert_eq!(composed_styling.get_effect(Effect::Bold), false);
                    assert_eq!(composed_styling.get(Effect::Bold), false);
                }

                {
                    let composed_styling = $empty_composed_styling.set(Effect::Bold, true);
                    assert_eq!(composed_styling, $empty_composed_styling.bold());
                    assert_eq!(composed_styling.get_effect(Effect::Bold), true);
                    assert_eq!(composed_styling.get(Effect::Bold), true);

                    let composed_styling = composed_styling.unset(Effect::Bold);
                    assert_eq!(composed_styling, $empty_composed_styling);
                    assert_eq!(composed_styling.get_effect(Effect::Bold), false);
                    assert_eq!(composed_styling.get(Effect::Bold), false);
                }
            }

            #[test]
            fn get_effects() {
                let composed_styling = $empty_composed_styling.bold().italic().underline();
                let mut effects = composed_styling.get_effects();

                assert_eq!(effects.next(), Some(Effect::Bold));
                assert_eq!(effects.next(), Some(Effect::Italic));
                assert_eq!(effects.next(), Some(Effect::Underline));
                assert_eq!(effects.next(), None);
            }

            #[test]
            fn underline_styles() {
                let composed_styling = $empty_composed_styling;
                assert_eq!(composed_styling.get_underline_style(), None);
                assert_eq!(composed_styling.get(Underline), None);

                {
                    let composed_styling =
                        $empty_composed_styling.set_underline_style(Some(UnderlineStyle::Solid));
                    assert_eq!(composed_styling, $empty_composed_styling.underline());
                    assert_eq!(
                        composed_styling.get_underline_style(),
                        Some(UnderlineStyle::Solid)
                    );
                    assert_eq!(composed_styling.get(Underline), Some(UnderlineStyle::Solid));

                    let composed_styling = composed_styling.set_underline_style(None);
                    assert_eq!(composed_styling, $empty_composed_styling);
                    assert_eq!(composed_styling.get_underline_style(), None);
                    assert_eq!(composed_styling.get(Underline), None);
                }

                {
                    let composed_styling =
                        $empty_composed_styling.set(Underline, Some(UnderlineStyle::Solid));
                    assert_eq!(composed_styling, $empty_composed_styling.underline());
                    assert_eq!(
                        composed_styling.get_underline_style(),
                        Some(UnderlineStyle::Solid)
                    );
                    assert_eq!(composed_styling.get(Underline), Some(UnderlineStyle::Solid));

                    let composed_styling = composed_styling.unset(Underline);
                    assert_eq!(composed_styling, $empty_composed_styling);
                    assert_eq!(composed_styling.get_underline_style(), None);
                    assert_eq!(composed_styling.get(Underline), None);
                }

                {
                    let composed_styling = $empty_composed_styling.set(UnderlineStyle::Solid, true);
                    assert_eq!(composed_styling, $empty_composed_styling.underline());
                    assert_eq!(
                        composed_styling.get_underline_style(),
                        Some(UnderlineStyle::Solid)
                    );
                    assert_eq!(composed_styling.get(Underline), Some(UnderlineStyle::Solid));
                    assert_eq!(composed_styling.get(UnderlineStyle::Solid), true);

                    let composed_styling = composed_styling.unset(UnderlineStyle::Solid);
                    assert_eq!(composed_styling, $empty_composed_styling);
                    assert_eq!(composed_styling.get_underline_style(), None);
                    assert_eq!(composed_styling.get(Underline), None);
                    assert_eq!(composed_styling.get(UnderlineStyle::Solid), false);
                }

                {
                    let composed_styling = $empty_composed_styling.set(UnderlineStyle::Solid, true);
                    assert_eq!(composed_styling, $empty_composed_styling.underline());
                    assert_eq!(
                        composed_styling.get_underline_style(),
                        Some(UnderlineStyle::Solid)
                    );
                    assert_eq!(composed_styling.get(Underline), Some(UnderlineStyle::Solid));
                    assert_eq!(composed_styling.get(UnderlineStyle::Solid), true);

                    let composed_styling = composed_styling.set(UnderlineStyle::Solid, false);
                    assert_eq!(composed_styling, $empty_composed_styling);
                    assert_eq!(composed_styling.get_underline_style(), None);
                    assert_eq!(composed_styling.get(Underline), None);
                    assert_eq!(composed_styling.get(UnderlineStyle::Solid), false);
                }

                {
                    let composed_styling =
                        $empty_composed_styling.set_effect(UnderlineStyle::Solid, true);
                    assert_eq!(composed_styling, $empty_composed_styling.underline());
                    assert_eq!(
                        composed_styling.get_underline_style(),
                        Some(UnderlineStyle::Solid)
                    );
                    assert_eq!(composed_styling.get(Underline), Some(UnderlineStyle::Solid));
                    assert_eq!(composed_styling.get_effect(UnderlineStyle::Solid), true);

                    let composed_styling =
                        composed_styling.set_effect(UnderlineStyle::Solid, false);
                    assert_eq!(composed_styling, $empty_composed_styling);
                    assert_eq!(composed_styling.get_underline_style(), None);
                    assert_eq!(composed_styling.get(Underline), None);
                    assert_eq!(composed_styling.get_effect(UnderlineStyle::Solid), false);
                }
            }

            macro_rules! assert_targeted_color {
                ($color_target:expr, $method:ident) => {
                    let empty_composed_styling = $empty_composed_styling;
                    assert_eq!(empty_composed_styling.get_color($color_target), None);
                    assert_eq!(empty_composed_styling.get($color_target), None);

                    let composed_styling =
                        $empty_composed_styling.set_color($color_target, Some(BasicColor::Red));
                    assert_eq!(
                        composed_styling,
                        $empty_composed_styling.$method(BasicColor::Red)
                    );
                    assert_eq!(
                        composed_styling.get_color($color_target),
                        Some(BasicColor::Red.to_color())
                    );
                    assert_eq!(
                        composed_styling.get($color_target),
                        Some(BasicColor::Red.to_color())
                    );

                    let composed_styling = $empty_composed_styling
                        .set($color_target, Some(BasicColor::Red.to_color()));
                    assert_eq!(
                        composed_styling,
                        $empty_composed_styling.$method(BasicColor::Red)
                    );
                    assert_eq!(
                        composed_styling.get_color($color_target),
                        Some(BasicColor::Red.to_color())
                    );
                    assert_eq!(
                        composed_styling.get($color_target),
                        Some(BasicColor::Red.to_color())
                    );

                    let composed_styling =
                        $empty_composed_styling.set_color($color_target, Some(BasicColor::Red));

                    {
                        let empty_composed_styling =
                            composed_styling.set_color($color_target, None::<Color>);
                        assert_eq!(empty_composed_styling, $empty_composed_styling);
                        assert_eq!(empty_composed_styling.get_color($color_target), None);
                        assert_eq!(empty_composed_styling.get($color_target), None);
                    }

                    {
                        let empty_composed_styling = composed_styling.unset($color_target);
                        assert_eq!(empty_composed_styling, $empty_composed_styling);
                        assert_eq!(empty_composed_styling.get_color($color_target), None);
                        assert_eq!(empty_composed_styling.get($color_target), None);
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
pub(crate) use test_composed_styling_type;
