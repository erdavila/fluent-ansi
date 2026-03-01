macro_rules! test_composed {
    ($empty_composed:expr) => {
        mod composed {
            use crate::{color::*, *};

            #[test]
            fn effects() {
                let composed = $empty_composed;
                assert_eq!(composed.get_effect(Effect::Bold), false);
                assert_eq!(composed.get(Effect::Bold), false);
                assert_eq!(composed.get_effect(Effect::Italic), false);
                assert_eq!(composed.get(Effect::Italic), false);

                {
                    let composed = $empty_composed.set_effect(Effect::Bold, true);
                    assert_eq!(composed, $empty_composed.bold());
                    assert_eq!(composed.get_effect(Effect::Bold), true);
                    assert_eq!(composed.get(Effect::Bold), true);
                    assert_eq!(composed.get_effect(Effect::Italic), false);
                    assert_eq!(composed.get(Effect::Italic), false);

                    let composed = composed.set_effect(Effect::Bold, false);
                    assert_eq!(composed, $empty_composed);
                    assert_eq!(composed.get_effect(Effect::Bold), false);
                    assert_eq!(composed.get(Effect::Bold), false);
                }

                {
                    let composed = $empty_composed.set(Effect::Bold, true);
                    assert_eq!(composed, $empty_composed.bold());
                    assert_eq!(composed.get_effect(Effect::Bold), true);
                    assert_eq!(composed.get(Effect::Bold), true);

                    {
                        let composed = composed.remove(Effect::Bold);
                        assert_eq!(composed, $empty_composed);
                        assert_eq!(composed.get_effect(Effect::Bold), false);
                        assert_eq!(composed.get(Effect::Bold), false);
                    }

                    {
                        let composed = composed - Effect::Bold;
                        assert_eq!(composed, $empty_composed);
                        assert_eq!(composed.get_effect(Effect::Bold), false);
                        assert_eq!(composed.get(Effect::Bold), false);
                    }
                }
            }

            #[test]
            fn get_effects() {
                let composed = $empty_composed.bold().italic().solid_underline();
                let mut effects = composed.get_effects();

                assert_eq!(effects.next(), Some(Effect::Bold));
                assert_eq!(effects.next(), Some(Effect::Italic));
                assert_eq!(effects.next(), Some(Effect::SolidUnderline));
                assert_eq!(effects.next(), None);
            }

            #[test]
            fn underline_effects() {
                let composed = $empty_composed;
                assert_eq!(composed.get_underline_effect(), None);
                assert_eq!(composed.get(UnderlineStyle), None);

                {
                    let composed =
                        $empty_composed.set_underline_effect(Some(UnderlineEffect::Solid));
                    assert_eq!(composed, $empty_composed.solid_underline());
                    assert_eq!(
                        composed.get_underline_effect(),
                        Some(UnderlineEffect::Solid)
                    );
                    assert_eq!(
                        composed.get(UnderlineStyle),
                        Some(UnderlineEffect::Solid)
                    );

                    let composed = composed.set_underline_effect(None);
                    assert_eq!(composed, $empty_composed);
                    assert_eq!(composed.get_underline_effect(), None);
                    assert_eq!(composed.get(UnderlineStyle), None);
                }

                {
                    let composed =
                        $empty_composed.set(UnderlineStyle, Some(UnderlineEffect::Solid));
                    assert_eq!(composed, $empty_composed.solid_underline());
                    assert_eq!(
                        composed.get_underline_effect(),
                        Some(UnderlineEffect::Solid)
                    );
                    assert_eq!(
                        composed.get(UnderlineStyle),
                        Some(UnderlineEffect::Solid)
                    );

                    {
                        let composed = composed.remove(UnderlineStyle);
                        assert_eq!(composed, $empty_composed);
                        assert_eq!(composed.get_underline_effect(), None);
                        assert_eq!(composed.get(UnderlineStyle), None);
                    }

                    {
                        let composed = composed - UnderlineStyle;
                        assert_eq!(composed, $empty_composed);
                        assert_eq!(composed.get_underline_effect(), None);
                        assert_eq!(composed.get(UnderlineStyle), None);
                    }
                }

                {
                    let composed =
                        $empty_composed.set(UnderlineEffect::Solid, true);
                    assert_eq!(composed, $empty_composed.solid_underline());
                    assert_eq!(
                        composed.get_underline_effect(),
                        Some(UnderlineEffect::Solid)
                    );
                    assert_eq!(
                        composed.get(UnderlineStyle),
                        Some(UnderlineEffect::Solid)
                    );
                    assert_eq!(composed.get(UnderlineEffect::Solid), true);

                    {
                        let composed = composed.remove(UnderlineEffect::Solid);
                        assert_eq!(composed, $empty_composed);
                        assert_eq!(composed.get_underline_effect(), None);
                        assert_eq!(composed.get(UnderlineStyle), None);
                        assert_eq!(composed.get(UnderlineEffect::Solid), false);
                    }

                    {
                        let composed = composed - UnderlineEffect::Solid;
                        assert_eq!(composed, $empty_composed);
                        assert_eq!(composed.get_underline_effect(), None);
                        assert_eq!(composed.get(UnderlineStyle), None);
                        assert_eq!(composed.get(UnderlineEffect::Solid), false);
                    }
                }

                {
                    let composed =
                        $empty_composed.set(UnderlineEffect::Solid, true);
                    assert_eq!(composed, $empty_composed.solid_underline());
                    assert_eq!(
                        composed.get_underline_effect(),
                        Some(UnderlineEffect::Solid)
                    );
                    assert_eq!(
                        composed.get(UnderlineStyle),
                        Some(UnderlineEffect::Solid)
                    );
                    assert_eq!(composed.get(UnderlineEffect::Solid), true);

                    let composed = composed.set(UnderlineEffect::Solid, false);
                    assert_eq!(composed, $empty_composed);
                    assert_eq!(composed.get_underline_effect(), None);
                    assert_eq!(composed.get(UnderlineStyle), None);
                    assert_eq!(composed.get(UnderlineEffect::Solid), false);
                }

                {
                    let composed =
                        $empty_composed.set_effect(UnderlineEffect::Solid, true);
                    assert_eq!(composed, $empty_composed.solid_underline());
                    assert_eq!(
                        composed.get_underline_effect(),
                        Some(UnderlineEffect::Solid)
                    );
                    assert_eq!(
                        composed.get(UnderlineStyle),
                        Some(UnderlineEffect::Solid)
                    );
                    assert_eq!(composed.get_effect(UnderlineEffect::Solid), true);

                    let composed =
                        composed.set_effect(UnderlineEffect::Solid, false);
                    assert_eq!(composed, $empty_composed);
                    assert_eq!(composed.get_underline_effect(), None);
                    assert_eq!(composed.get(UnderlineStyle), None);
                    assert_eq!(composed.get_effect(UnderlineEffect::Solid), false);
                }
            }

            macro_rules! assert_targeted_color {
                ($color_target:expr, $method:ident) => {
                    let empty_composed = $empty_composed;
                    assert_eq!(empty_composed.get_color($color_target), None);
                    assert_eq!(empty_composed.get($color_target), None);

                    let composed =
                        $empty_composed.set_color($color_target, Some(BasicColor::Red));
                    assert_eq!(
                        composed,
                        $empty_composed.$method(BasicColor::Red)
                    );
                    assert_eq!(
                        composed.get_color($color_target),
                        Some(BasicColor::Red.to_color())
                    );
                    assert_eq!(
                        composed.get($color_target),
                        Some(BasicColor::Red.to_color())
                    );

                    let composed = $empty_composed
                        .set($color_target, Some(BasicColor::Red.to_color()));
                    assert_eq!(
                        composed,
                        $empty_composed.$method(BasicColor::Red)
                    );
                    assert_eq!(
                        composed.get_color($color_target),
                        Some(BasicColor::Red.to_color())
                    );
                    assert_eq!(
                        composed.get($color_target),
                        Some(BasicColor::Red.to_color())
                    );

                    let composed =
                        $empty_composed.set_color($color_target, Some(BasicColor::Red));

                    {
                        let empty_composed =
                            composed.set_color($color_target, None::<Color>);
                        assert_eq!(empty_composed, $empty_composed);
                        assert_eq!(empty_composed.get_color($color_target), None);
                        assert_eq!(empty_composed.get($color_target), None);
                    }

                    {
                        let empty_composed = composed.remove($color_target);
                        assert_eq!(empty_composed, $empty_composed);
                        assert_eq!(empty_composed.get_color($color_target), None);
                        assert_eq!(empty_composed.get($color_target), None);
                    }

                    {
                        let empty_composed = composed - $color_target;
                        assert_eq!(empty_composed, $empty_composed);
                        assert_eq!(empty_composed.get_color($color_target), None);
                        assert_eq!(empty_composed.get($color_target), None);
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

            #[test]
            fn merge_style() {
                macro_rules! assert_merge_style {
                    ($initial:expr, $style:expr; $expected:expr) => {{
                        let merged = $initial.merge_style($style);
                        assert_eq!(merged, $expected);
                    }};
                }

                assert_merge_style!(
                    $empty_composed,
                    Style::new();
                    $empty_composed
                );

                assert_merge_style!(
                    $empty_composed
                        .bold()
                        .solid_underline()
                        .foreground(Color::RED)
                        .background(Color::GREEN),
                    Style::new();
                    $empty_composed
                        .bold()
                        .solid_underline()
                        .foreground(Color::RED)
                        .background(Color::GREEN)
                );

                assert_merge_style!(
                    $empty_composed,
                    Style::new()
                        .italic()
                        .dashed_underline()
                        .background(Color::BLUE)
                        .underline_color(Color::YELLOW);
                    $empty_composed
                        .italic()
                        .dashed_underline()
                        .background(Color::BLUE)
                        .underline_color(Color::YELLOW)
                );

                assert_merge_style!(
                    $empty_composed
                        .bold()
                        .solid_underline()
                        .foreground(Color::RED)
                        .background(Color::GREEN),
                    Style::new()
                        .italic()
                        .dashed_underline()  // overrides the solid underline
                        .background(Color::BLUE)  // overrides the green background
                        .underline_color(Color::YELLOW);
                    $empty_composed
                        .bold()
                        .italic()
                        .dashed_underline()
                        .foreground(Color::RED)
                        .background(Color::BLUE)
                        .underline_color(Color::YELLOW)
                );

                for self_enabled in [false, true] {
                    for other_enabled in [false, true] {
                        assert_merge_style!(
                            $empty_composed
                                .set_enabled(self_enabled),
                            Style::new()
                                .set_enabled(other_enabled);
                            $empty_composed
                                .set_enabled(other_enabled)
                        );
                    }
                }
            }
        }
    };
}
pub(crate) use test_composed;
