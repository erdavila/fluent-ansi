macro_rules! impl_fluent_type {
    ($name:ident {
        args: [$self:ident];
        to_style: $to_style:tt
    }) => {
        impl $name {
            $crate::impl_macros::fluent::impl_fluent_methods!();

            /// Applies the styling to the given content, returning a [`Styled<C>`](crate::Styled) instance.
            #[must_use]
            pub fn applied_to<C: core::fmt::Display>(self, content: C) -> $crate::Styled<C> {
                let style = $crate::Style::from(self);
                $crate::Styled::new(content).with_style(style)
            }
        }

        $crate::impl_macros::fluent::__impl_fluent_type__to_style!($name, $self, $to_style);
    };
}
pub(crate) use impl_fluent_type;

macro_rules! impl_fluent_methods {
    () => {
        $crate::impl_macros::fluent::impl_fluent_methods! {
            type StyleSet = $crate::Style;
            args: [self];
            to_style_set: { self.to_style() }
        }
    };

    {
        type StyleSet = $style_set_type:ty;
        args: [$self:ident];
        to_style_set: $to_style_set:block
    } => {
        /// Sets the bold effect.
        #[must_use]
        pub fn bold(self) -> $style_set_type {
            self.effect($crate::Effect::Bold)
        }

        /// Sets the faint effect.
        #[must_use]
        pub fn faint(self) -> $style_set_type {
            self.effect($crate::Effect::Faint)
        }

        /// Sets the italic effect.
        #[must_use]
        pub fn italic(self) -> $style_set_type {
            self.effect($crate::Effect::Italic)
        }

        /// Sets the solid underline effect.
        #[must_use]
        pub fn underline(self) -> $style_set_type {
            self.effect($crate::Effect::Underline)
        }

        /// Sets the curly underline effect.
        #[must_use]
        pub fn curly_underline(self) -> $style_set_type {
            self.effect($crate::Effect::CurlyUnderline)
        }

        /// Sets the dotted underline effect.
        #[must_use]
        pub fn dotted_underline(self) -> $style_set_type {
            self.effect($crate::Effect::DottedUnderline)
        }

        /// Sets the dashed underline effect.
        #[must_use]
        pub fn dashed_underline(self) -> $style_set_type {
            self.effect($crate::Effect::DashedUnderline)
        }

        /// Sets the blink effect.
        #[must_use]
        pub fn blink(self) -> $style_set_type {
            self.effect($crate::Effect::Blink)
        }

        /// Sets the reverse effect.
        #[must_use]
        pub fn reverse(self) -> $style_set_type {
            self.effect($crate::Effect::Reverse)
        }

        /// Sets the conceal effect.
        #[must_use]
        pub fn conceal(self) -> $style_set_type {
            self.effect($crate::Effect::Conceal)
        }

        /// Sets the strikethrough effect.
        #[must_use]
        pub fn strikethrough(self) -> $style_set_type {
            self.effect($crate::Effect::Strikethrough)
        }

        /// Sets the double underline effect.
        #[must_use]
        pub fn double_underline(self) -> $style_set_type {
            self.effect($crate::Effect::DoubleUnderline)
        }

        /// Sets the overline effect.
        #[must_use]
        pub fn overline(self) -> $style_set_type {
            self.effect($crate::Effect::Overline)
        }

        /// Sets the given effect.
        #[must_use]
        pub fn effect(self, effect: impl Into<$crate::Effect>) -> $style_set_type {
            use crate::StyleSet as _;
            self.to_style_set().set_effect(effect, true)
        }

        /// Sets the underline style.
        #[must_use]
        pub fn underline_style(self, underline_style: $crate::UnderlineStyle) -> $style_set_type {
            self.effect(underline_style)
        }

        /// Sets the foreground color.
        #[must_use]
        pub fn fg(self, color: impl Into<$crate::color::Color>) -> $style_set_type {
            self.color($crate::TargetedColor::new_for_fg(color))
        }

        /// Sets the background color.
        #[must_use]
        pub fn bg(self, color: impl Into<$crate::color::Color>) -> $style_set_type {
            self.color($crate::TargetedColor::new_for_bg(color))
        }

        /// Sets the underline color.
        #[must_use]
        pub fn underline_color(self, color: impl Into<$crate::color::Color>) -> $style_set_type {
            self.color($crate::TargetedColor::new_for_underline(color))
        }

        /// Sets the given color in a target.
        #[must_use]
        pub fn color(
            self,
            targeted_color: impl Into<$crate::TargetedColor>,
        ) -> $style_set_type {
            use crate::StyleSet as _;
            let targeted_color = targeted_color.into();
            self.to_style_set().set_color(
                targeted_color.get_target(),
                Some(targeted_color.get_color()),
            )
        }

        /// Adds the given element to the style.
        #[expect(clippy::should_implement_trait)]
        #[must_use]
        pub fn add(self, element: impl $crate::StyleElement) -> $style_set_type {
            let style_set = self.to_style_set();
            element.add_to(style_set)
        }

        #[allow(clippy::wrong_self_convention)]
        #[must_use]
        fn to_style_set($self) -> $style_set_type $to_style_set
    };
}
pub(crate) use impl_fluent_methods;

macro_rules! __impl_fluent_type__to_style {
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
pub(crate) use __impl_fluent_type__to_style;
