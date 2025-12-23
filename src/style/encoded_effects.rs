use crate::{AllEffects, Effect, UnderlineStyle};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) struct EncodedEffects(u16);

impl EncodedEffects {
    #[must_use]
    pub(crate) const fn new() -> Self {
        Self(0)
    }

    #[must_use]
    pub(crate) fn set(self, effect: Effect, value: bool) -> Self {
        if value {
            self.add(effect)
        } else {
            self.remove(effect)
        }
    }

    #[must_use]
    fn add(self, effect: Effect) -> Self {
        let underline_style = UnderlineStyle::all().find(|t| t.to_effect() == effect);
        if let Some(underline_style) = underline_style {
            self.set_underline(Some(underline_style))
        } else {
            self.set_bit(effect)
        }
    }

    #[must_use]
    fn remove(self, effect: Effect) -> Self {
        self.clear_bit(effect)
    }

    #[must_use]
    pub(crate) fn set_underline(self, underline_style: Option<UnderlineStyle>) -> Self {
        let encoded_effects = self.remove_underline();
        if let Some(underline_style) = underline_style {
            encoded_effects.set_bit(underline_style.to_effect())
        } else {
            encoded_effects
        }
    }

    #[must_use]
    fn remove_underline(self) -> Self {
        let mut encoded_effects = self;
        for underline_style in UnderlineStyle::all() {
            encoded_effects = encoded_effects.clear_bit(underline_style.to_effect());
        }
        encoded_effects
    }

    #[must_use]
    pub(crate) fn get(self, effect: Effect) -> bool {
        self.0 & Self::bit_mask(effect) != 0
    }

    #[must_use]
    pub(crate) fn get_effects(self) -> GetEffects {
        GetEffects {
            inner: Effect::all(),
            encoded_effects: self,
        }
    }

    #[must_use]
    fn set_bit(self, effect: Effect) -> Self {
        let bits = self.0 | Self::bit_mask(effect);
        Self(bits)
    }

    #[must_use]
    fn clear_bit(self, effect: Effect) -> Self {
        let bits = self.0 & !Self::bit_mask(effect);
        Self(bits)
    }

    #[must_use]
    fn bit_mask(effect: Effect) -> u16 {
        let bit_index = effect as u16;
        1 << bit_index
    }
}

/// An iterator over the effects that are currently set.
pub struct GetEffects {
    inner: AllEffects,
    encoded_effects: EncodedEffects,
}
impl Iterator for GetEffects {
    type Item = Effect;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .by_ref()
            .find(|&effect| self.encoded_effects.get(effect))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn underline_effects() -> impl Iterator<Item = Effect> {
        UnderlineStyle::all().map(UnderlineStyle::to_effect)
    }

    #[test]
    fn default() {
        let effects = EncodedEffects::default();

        for effect in Effect::all() {
            assert_eq!(effects.get(effect), false);
        }
    }

    #[test]
    fn set_true() {
        for added_effect in Effect::all() {
            let effects = EncodedEffects(0).set(added_effect, true);

            for checked_effect in Effect::all() {
                assert_eq!(
                    effects.get(checked_effect),
                    added_effect == checked_effect,
                    "{added_effect:?} {checked_effect:?}"
                );
            }
        }
    }

    #[test]
    fn set_false() {
        for removed_effect in Effect::all() {
            let effects = EncodedEffects(!0).set(removed_effect, false);

            for checked_effect in Effect::all() {
                assert_eq!(
                    effects.get(checked_effect),
                    removed_effect != checked_effect,
                    "{removed_effect:?} {checked_effect:?}"
                );
            }
        }
    }

    #[test]
    fn set_underline_true() {
        for initial_effect in underline_effects() {
            // Add some effect
            let encoded_effects = EncodedEffects::default().set(initial_effect, true);

            for other_effect in underline_effects() {
                if other_effect == initial_effect {
                    continue;
                }

                // Add other effect
                let encoded_effects = encoded_effects.set(other_effect, true);

                // Only the most recently added effect should be set
                assert!(
                    !encoded_effects.get(initial_effect),
                    "{initial_effect:?} should not be set"
                );
                assert!(
                    encoded_effects.get(other_effect),
                    "{other_effect:?} should be set"
                );
            }
        }
    }

    #[test]
    fn set_underline_some() {
        for initial_style in UnderlineStyle::all() {
            // Add some effect
            let encoded_effects = EncodedEffects::default().set(initial_style.to_effect(), true);

            for other_style in UnderlineStyle::all() {
                if other_style == initial_style {
                    continue;
                }

                // Add other effect
                let encoded_effects = encoded_effects.set_underline(Some(other_style));

                // Only the most recently added effect should be set
                assert!(
                    !encoded_effects.get(initial_style.to_effect()),
                    "{initial_style:?} should not be set"
                );
                assert!(
                    encoded_effects.get(other_style.to_effect()),
                    "{other_style:?} should be set"
                );
            }
        }
    }

    macro_rules! test_clear_underline {
        ($method:ident ( $( $arg:expr )?) ) => {
            for initial_style in UnderlineStyle::all() {
                // Add some effect
                let encoded_effects = EncodedEffects::default().set(initial_style.to_effect(), true);

                // Clear underline effect
                let encoded_effects = encoded_effects.$method( $( $arg )? );

                for checked_style in UnderlineStyle::all() {
                    assert!(
                        !encoded_effects.get(checked_style.to_effect()),
                        "{initial_style:?} should not be set"
                    );
                }
            }
        };
    }

    #[test]
    fn set_underline_none() {
        test_clear_underline!(set_underline(None));
    }

    #[test]
    fn remove_underline() {
        test_clear_underline!(remove_underline());
    }

    #[test]
    fn get_effects() {
        let effects = EncodedEffects::default()
            .add(Effect::Bold)
            .add(Effect::Underline);

        let mut get_effects = effects.get_effects();

        assert_eq!(get_effects.next(), Some(Effect::Bold));
        assert_eq!(get_effects.next(), Some(Effect::Underline));
        assert_eq!(get_effects.next(), None);
    }
}
