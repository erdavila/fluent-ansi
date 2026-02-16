macro_rules! impl_additive_styling_type {
    ($name:ident {
        args: [$self:ident];
        to_style: $to_style:tt
    }) => {
        impl $name {
            $crate::impl_macros::additive_styling::impl_additive_styling_methods!();

            /// Applies the styling to the given content, returning a [`Styled<C>`](crate::Styled) instance.
            #[must_use]
            pub fn applied_to<C: core::fmt::Display>(self, content: C) -> $crate::Styled<C> {
                let style = $crate::Style::from(self);
                $crate::Styled::new(content).with_style(style)
            }
        }

        $crate::impl_macros::additive_styling::__impl_additive_styling_type__to_style!(
            $name, $self, $to_style
        );
    };
}
pub(crate) use impl_additive_styling_type;

macro_rules! impl_additive_styling_methods {
    () => {
        $crate::impl_macros::additive_styling::impl_additive_styling_methods! {
            type ComposedStyling = $crate::Style;
            args: [self];
            to_composed_styling: { self.to_style() }
        }
    };

    {
        type ComposedStyling = $composed_styling_type:ty;
        args: [$self:ident];
        to_composed_styling: $to_composed_styling:block
    } => {
        /// Sets the bold effect.
        #[must_use]
        pub fn bold(self) -> $composed_styling_type {
            self.effect($crate::Effect::Bold)
        }

        /// Sets the faint effect.
        #[must_use]
        pub fn faint(self) -> $composed_styling_type {
            self.effect($crate::Effect::Faint)
        }

        /// Sets the italic effect.
        #[must_use]
        pub fn italic(self) -> $composed_styling_type {
            self.effect($crate::Effect::Italic)
        }

        /// An alias for [`Self::solid_underline()`].
        #[must_use]
        pub fn underline(self) -> $composed_styling_type {
            self.solid_underline()
        }

        /// Sets the solid underline effect.
        #[must_use]
        pub fn solid_underline(self) -> $composed_styling_type {
            self.effect($crate::Effect::SolidUnderline)
        }

        /// Sets the curly underline effect.
        #[must_use]
        pub fn curly_underline(self) -> $composed_styling_type {
            self.effect($crate::Effect::CurlyUnderline)
        }

        /// Sets the dotted underline effect.
        #[must_use]
        pub fn dotted_underline(self) -> $composed_styling_type {
            self.effect($crate::Effect::DottedUnderline)
        }

        /// Sets the dashed underline effect.
        #[must_use]
        pub fn dashed_underline(self) -> $composed_styling_type {
            self.effect($crate::Effect::DashedUnderline)
        }

        /// Sets the blink effect.
        #[must_use]
        pub fn blink(self) -> $composed_styling_type {
            self.effect($crate::Effect::Blink)
        }

        /// Sets the reverse effect.
        #[must_use]
        pub fn reverse(self) -> $composed_styling_type {
            self.effect($crate::Effect::Reverse)
        }

        /// Sets the conceal effect.
        #[must_use]
        pub fn conceal(self) -> $composed_styling_type {
            self.effect($crate::Effect::Conceal)
        }

        /// Sets the strikethrough effect.
        #[must_use]
        pub fn strikethrough(self) -> $composed_styling_type {
            self.effect($crate::Effect::Strikethrough)
        }

        /// Sets the double underline effect.
        #[must_use]
        pub fn double_underline(self) -> $composed_styling_type {
            self.effect($crate::Effect::DoubleUnderline)
        }

        /// Sets the overline effect.
        #[must_use]
        pub fn overline(self) -> $composed_styling_type {
            self.effect($crate::Effect::Overline)
        }

        /// Sets the given effect.
        #[must_use]
        pub fn effect(self, effect: impl Into<$crate::Effect>) -> $composed_styling_type {
            use $crate::traits::Composed as _;
            self.to_composed_styling().set_effect(effect, true)
        }

        /// Sets the underline effect.
        #[must_use]
        pub fn underline_effect(self, underline_effect: $crate::UnderlineEffect) -> $composed_styling_type {
            self.effect(underline_effect)
        }

        /// Sets the foreground color.
        #[must_use]
        pub fn foreground(self, color: impl Into<$crate::color::Color>) -> $composed_styling_type {
            self.color($crate::TargetedColor::new_for_fg(color))
        }

        /// Alias for [`Self::foreground()`].
        #[must_use]
        pub fn fg(self, color: impl Into<$crate::color::Color>) -> $composed_styling_type {
            self.foreground(color)
        }

        /// Sets the background color.
        #[must_use]
        pub fn background(self, color: impl Into<$crate::color::Color>) -> $composed_styling_type {
            self.color($crate::TargetedColor::new_for_bg(color))
        }

        /// Alias for [`Self::background()`].
        #[must_use]
        pub fn bg(self, color: impl Into<$crate::color::Color>) -> $composed_styling_type {
            self.background(color)
        }

        /// Sets the underline color.
        #[must_use]
        pub fn underline_color(self, color: impl Into<$crate::color::Color>) -> $composed_styling_type {
            self.color($crate::TargetedColor::new_for_underline(color))
        }

        /// Sets the given color in a target.
        #[must_use]
        pub fn color(
            self,
            targeted_color: impl Into<$crate::TargetedColor>,
        ) -> $composed_styling_type {
            use $crate::traits::Composed as _;
            let targeted_color = targeted_color.into();
            self.to_composed_styling().set_color(
                targeted_color.get_target(),
                Some(targeted_color.get_color()),
            )
        }

        /// Adds the given element to the style.
        #[expect(clippy::should_implement_trait)]
        #[must_use]
        pub fn add(self, element: impl $crate::StylingElement<$composed_styling_type>) -> $composed_styling_type {
            let composed_styling = self.to_composed_styling();
            element.add_to(composed_styling)
        }

        #[allow(clippy::wrong_self_convention)]
        #[must_use]
        fn to_composed_styling($self) -> $composed_styling_type $to_composed_styling
    };
}
pub(crate) use impl_additive_styling_methods;

macro_rules! __impl_additive_styling_type__to_style {
    ($name:ident, $self:ident, SELF) => {
        // Defines only the to_style method
        impl $name {
            #[doc = r"Convert this type into a [`Style`](crate::Style)."]
            #[must_use]
            pub fn to_style(self) -> Style {
                self
            }
        }
    };

    ($name:ident, $self:ident, $to_style:tt ) => {
        // Defines the to_style method and impl From<$name> for Style
        $crate::impl_macros::from_to::impl_from_to!(
            #[doc = r"Converts the type into a [`Style`](crate::Style)."]
            fn to_style($self: $name) -> $crate::Style $to_style
        );
    };
}
pub(crate) use __impl_additive_styling_type__to_style;
