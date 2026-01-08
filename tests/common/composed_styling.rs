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

                    let composed_styling = composed_styling.remove(Effect::Bold);
                    assert_eq!(composed_styling, $empty_composed_styling);
                    assert_eq!(composed_styling.get_effect(Effect::Bold), false);
                    assert_eq!(composed_styling.get(Effect::Bold), false);
                }
            }

            #[test]
            fn get_effects() {
                let composed_styling = $empty_composed_styling.bold().italic().solid_underline();
                let mut effects = composed_styling.get_effects();

                assert_eq!(effects.next(), Some(Effect::Bold));
                assert_eq!(effects.next(), Some(Effect::Italic));
                assert_eq!(effects.next(), Some(Effect::SolidUnderline));
                assert_eq!(effects.next(), None);
            }

            #[test]
            fn underline_effects() {
                let composed_styling = $empty_composed_styling;
                assert_eq!(composed_styling.get_underline_effect(), None);
                assert_eq!(composed_styling.get(UnderlineStyle), None);

                {
                    let composed_styling =
                        $empty_composed_styling.set_underline_effect(Some(UnderlineEffect::Solid));
                    assert_eq!(composed_styling, $empty_composed_styling.solid_underline());
                    assert_eq!(
                        composed_styling.get_underline_effect(),
                        Some(UnderlineEffect::Solid)
                    );
                    assert_eq!(
                        composed_styling.get(UnderlineStyle),
                        Some(UnderlineEffect::Solid)
                    );

                    let composed_styling = composed_styling.set_underline_effect(None);
                    assert_eq!(composed_styling, $empty_composed_styling);
                    assert_eq!(composed_styling.get_underline_effect(), None);
                    assert_eq!(composed_styling.get(UnderlineStyle), None);
                }

                {
                    let composed_styling =
                        $empty_composed_styling.set(UnderlineStyle, Some(UnderlineEffect::Solid));
                    assert_eq!(composed_styling, $empty_composed_styling.solid_underline());
                    assert_eq!(
                        composed_styling.get_underline_effect(),
                        Some(UnderlineEffect::Solid)
                    );
                    assert_eq!(
                        composed_styling.get(UnderlineStyle),
                        Some(UnderlineEffect::Solid)
                    );

                    let composed_styling = composed_styling.remove(UnderlineStyle);
                    assert_eq!(composed_styling, $empty_composed_styling);
                    assert_eq!(composed_styling.get_underline_effect(), None);
                    assert_eq!(composed_styling.get(UnderlineStyle), None);
                }

                {
                    let composed_styling =
                        $empty_composed_styling.set(UnderlineEffect::Solid, true);
                    assert_eq!(composed_styling, $empty_composed_styling.solid_underline());
                    assert_eq!(
                        composed_styling.get_underline_effect(),
                        Some(UnderlineEffect::Solid)
                    );
                    assert_eq!(
                        composed_styling.get(UnderlineStyle),
                        Some(UnderlineEffect::Solid)
                    );
                    assert_eq!(composed_styling.get(UnderlineEffect::Solid), true);

                    let composed_styling = composed_styling.remove(UnderlineEffect::Solid);
                    assert_eq!(composed_styling, $empty_composed_styling);
                    assert_eq!(composed_styling.get_underline_effect(), None);
                    assert_eq!(composed_styling.get(UnderlineStyle), None);
                    assert_eq!(composed_styling.get(UnderlineEffect::Solid), false);
                }

                {
                    let composed_styling =
                        $empty_composed_styling.set(UnderlineEffect::Solid, true);
                    assert_eq!(composed_styling, $empty_composed_styling.solid_underline());
                    assert_eq!(
                        composed_styling.get_underline_effect(),
                        Some(UnderlineEffect::Solid)
                    );
                    assert_eq!(
                        composed_styling.get(UnderlineStyle),
                        Some(UnderlineEffect::Solid)
                    );
                    assert_eq!(composed_styling.get(UnderlineEffect::Solid), true);

                    let composed_styling = composed_styling.set(UnderlineEffect::Solid, false);
                    assert_eq!(composed_styling, $empty_composed_styling);
                    assert_eq!(composed_styling.get_underline_effect(), None);
                    assert_eq!(composed_styling.get(UnderlineStyle), None);
                    assert_eq!(composed_styling.get(UnderlineEffect::Solid), false);
                }

                {
                    let composed_styling =
                        $empty_composed_styling.set_effect(UnderlineEffect::Solid, true);
                    assert_eq!(composed_styling, $empty_composed_styling.solid_underline());
                    assert_eq!(
                        composed_styling.get_underline_effect(),
                        Some(UnderlineEffect::Solid)
                    );
                    assert_eq!(
                        composed_styling.get(UnderlineStyle),
                        Some(UnderlineEffect::Solid)
                    );
                    assert_eq!(composed_styling.get_effect(UnderlineEffect::Solid), true);

                    let composed_styling =
                        composed_styling.set_effect(UnderlineEffect::Solid, false);
                    assert_eq!(composed_styling, $empty_composed_styling);
                    assert_eq!(composed_styling.get_underline_effect(), None);
                    assert_eq!(composed_styling.get(UnderlineStyle), None);
                    assert_eq!(composed_styling.get_effect(UnderlineEffect::Solid), false);
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
                        let empty_composed_styling = composed_styling.remove($color_target);
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
