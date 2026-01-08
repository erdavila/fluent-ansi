macro_rules! impl_composed_styling_methods {
    {
        args: [$self:ident, $effect:ident, $underline_effect:ident, $target:ident, $color:ident, $value:ident, $other:ident];
        example_variable: $example_variable:literal;

        set_effect: $set_effect:block
        get_effect: $get_effect:block
        get_effects: $get_effects:block
        set_underline_effect: $set_underline_effect:block
        get_underline_effect: $get_underline_effect:block
        set_color: $set_color:block
        get_color: $get_color:block
        merge_style: $merge_style:block
    } => {
        /// Sets the given effect to the specified value.
        #[must_use]
        pub fn set_effect($self, $effect: impl Into<Effect>, $value: bool) -> Self {
            $set_effect
        }

        /// Gets whether the given effect is set.
        #[must_use]
        pub fn get_effect(&$self, $effect: impl Into<Effect>) -> bool {
            $get_effect
        }

        /// Returns an iterator over the effects that are currently set.
        #[must_use]
        pub fn get_effects(&$self) -> GetEffects {
            $get_effects
        }

        /// Sets the underline effect.
        #[must_use]
        pub fn set_underline_effect($self, $underline_effect: Option<UnderlineEffect>) -> Self
            $set_underline_effect

        /// Gets the underline effect.
        #[must_use]
        pub fn get_underline_effect(&$self) -> Option<UnderlineEffect> {
            $get_underline_effect
        }

        /// Sets the color for the given color target.
        ///
        /// To clear the color for some color target, the color type must be specified in the `None` value.
        /// To help with that, the [`Color::none()`](Color::none) method can be used:
        ///
        #[doc = concat!(r"
```
# use fluent_ansi::{prelude::*, ColorTarget, Style};
# let ", $example_variable, r" = Style::new();
", $example_variable, r".set_color(ColorTarget::Foreground, None::<Color>);
// or
", $example_variable, r".set_color(ColorTarget::Foreground, Color::none());
```
")]
        ///
        #[must_use]
        pub fn set_color($self, $target: $crate::ColorTarget, $color: Option<impl Into<Color>>) -> Self {
            $set_color
        }

        /// Gets the color for the given color target.
        #[must_use]
        pub fn get_color(&$self, $target: $crate::ColorTarget) -> Option<Color> {
            $get_color
        }

        /// Sets the given attribute to the specified value.
        #[must_use]
        pub fn set<A: $crate::StylingAttribute<Self>>(self, attr: A, value: A::Value) -> Self {
            attr.set_in(self, value)
        }

        /// Gets the value of the given attribute.
        #[must_use]
        pub fn get<A: $crate::StylingAttribute<Self>>(&self, attr: A) -> A::Value {
            attr.get_from(self)
        }

        /// Clears the given attribute.
        #[must_use]
        pub fn remove<A: $crate::StylingAttribute<Self>>(self, attr: A) -> Self {
            attr.set_in(self, A::Value::default())
        }

        /// Merge styling from the `Style` argument.
        #[must_use]
        pub fn merge_style($self, $other: Style) -> Self {
            $merge_style
        }
    };
}
pub(crate) use impl_composed_styling_methods;
