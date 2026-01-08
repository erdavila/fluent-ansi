#![no_std]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
//! `fluent-ansi` is a library to handle ANSI escape sequences for the terminal.
//! It is `no_std`, and relies on the [`Display`](core::fmt::Display) trait to render the sequences.
//!
//! ```
//! use fluent_ansi::{prelude::*, Style, Styled};
//!
//! let style: Style = Color::RED.bold();
//! let styled: Styled<&str> = style.applied_to("Some content");
//!
//! println!("{}", styled);
//!
//! let content_with_escape_sequences = format!("{}", styled);
//! assert_eq!(content_with_escape_sequences, "\x1b[1;31mSome content\x1b[0m");
//! ```
//!
//! It has modular and composable values, and with its fluent methods, it provides several forms to
//! reach the same result. For instance, all the lines below result in the same [`Style`] value:
//!
//! ```
//! use fluent_ansi::{prelude::*, ColorTarget, Style, TargetedColor};
//!
//! let style: Style = Style::new().set(Effect::Bold, true).set(ColorTarget::Foreground, Some(Color::RED.to_color()));
//! let style: Style = Style::new().set_effect(Effect::Bold, true).set_color(ColorTarget::Foreground, Some(Color::RED));
//! let style: Style = Style::new().add(Effect::Bold).add(TargetedColor::new(Color::RED, ColorTarget::Foreground));
//! let style: Style = Style::new().effect(Effect::Bold).color(TargetedColor::new(Color::RED, ColorTarget::Foreground));
//! let style: Style = Style::new().bold().foreground(Color::RED);
//! let style: Style = Effect::Bold.foreground(Color::RED);
//! let style: Style = Color::RED.for_foreground().bold();
//! let style: Style = Color::RED.for_fg().bold();
//! ```
//!
//! All styling types are immutable and implement [`Copy`], except for [`Styled<C>`](Styled),
//! which is copiable only if its content `C` type is also copiable.
//!
//!
//! # Styling types
//!
//! The styling types are categorized according to the following:
//!
//! * [Styling element types]:
//!   * [Effect types](#effect-types):
//!     * [`Effect`]
//!     * [`UnderlineEffect`]
//!   * [Color types](#color-types):
//!     * [`TargetedColor`]
//!     * The color types in [`color`]
//! * [Composed styling types]:
//!   * [`Style`]
//!   * [`Styled<C>`]
//!
//!
//! ## Styling element types
//!
//! Each styling element type represents a single styling.
//! They can be used on their own or -- through their fluent methods -- combined with other styling elements, or applied to some content:
//!
//! ```
//! use fluent_ansi::prelude::*;
//!
//! assert_eq!(format!("{}", Effect::Bold), "\x1b[1m");
//! assert_eq!(format!("{}", Effect::Bold.foreground(Color::RED)), "\x1b[1;31m");
//! assert_eq!(format!("{}", Effect::Bold.applied_to("Some content")), "\x1b[1mSome content\x1b[0m");
//! ```
//!
//! When composed, they result in the [`Style`] [composed styling type].
//!
//! When applied to some content, they result in the [`Styled<C>`] [composed styling type].
//!
//!
//! ### Effect types
//!
//! An effect is an styling element type that may or may not be present. They correspond to the variants in the [`Effect`] enum.
//!
//! A subset of effects correspond to underline effects. They are mutually exclusive, meaning that when
//! one of them is set, any previously set underline effect is cleared.
//!
//! ```
//! use fluent_ansi::prelude::*;
//!
//! let style = Effect::Bold.add(Effect::DottedUnderline);
//! assert!(style.get_effect(Effect::Bold));
//! assert!(style.get_effect(Effect::DottedUnderline));
//!
//! let style = style.add(Effect::DashedUnderline);
//! assert!(style.get_effect(Effect::Bold));
//! assert!(!style.get_effect(Effect::DottedUnderline));
//! assert!(style.get_effect(Effect::DashedUnderline));
//! ```
//!
//! The [`UnderlineEffect`] enum variants represent the underline effects.
//!
//!
//! ### Color types
//!
//! There is a handful of color types, which are defined in the [`color`] module.
//!
//! A color is rendered in a [`ColorTarget`], which is [`Foreground`](ColorTarget::Foreground),
//! [`Background`](ColorTarget::Background) or [`Underline`](ColorTarget::Underline).
//!
//! The type [`TargetedColor`] associates a color with a [`ColorTarget`]:
//!
//! ```
//! use fluent_ansi::{prelude::*, TargetedColor};
//!
//! let red_foreground: TargetedColor = Color::RED.for_foreground();
//! assert_eq!(format!("{}", red_foreground.applied_to("Some content")), "\x1b[31mSome content\x1b[0m");
//!
//! let red_background: TargetedColor = Color::RED.for_background();
//! assert_eq!(format!("{}", red_background.applied_to("Some content")), "\x1b[41mSome content\x1b[0m");
//!
//! let red_underline: TargetedColor = Color::RED.for_underline();
//! assert_eq!(format!("{}", red_underline.applied_to("Some content")), "\x1b[58;5;1mSome content\x1b[0m");
//! ```
//!
//! By default, colors are rendered in the foreground:
//!
//! ```
//! use fluent_ansi::prelude::*;
//!
//! let rendered_1 = format!("{}", Color::RED.applied_to("Some content"));
//! let rendered_2 = format!("{}", Color::RED.for_foreground().applied_to("Some content"));
//! assert_eq!(rendered_1, rendered_2);
//! assert_eq!(rendered_1, "\x1b[31mSome content\x1b[0m");
//! ```
//!
//! You can refrain from mentioning the color target when using only in the foreground. But
//! if another color target is being used too, be explicit about the foreground target:
//!
//! ```
//! use fluent_ansi::prelude::*;
//!
//! // Only the foreground is set
//! let style = Effect::Bold.add(Color::RED);
//! let rendered = format!("{}", style.applied_to("Some content"));
//! assert_eq!(rendered, "\x1b[1;31mSome content\x1b[0m");
//!
//! // Both foreground and underline colors are set
//! let style = Effect::Bold
//!     .add(Color::RED.for_foreground())
//!     .add(Color::indexed(42).for_underline());
//! let rendered = format!("{}", style.applied_to("Some content"));
//! assert_eq!(rendered, "\x1b[1;31;58;5;42mSome content\x1b[0m");
//! ```
//!
//! ## Composed styling types
//!
//! [`Style`] is the result of composing [styling element] values. A [`Style`] can be used on its own or -- through
//! their methods -- compose with other styling elements, or applied to some content:
//!
//! ```
//! use fluent_ansi::{prelude::*, Style};
//!
//! let style: Style = Effect::Bold.foreground(Color::RED);
//! assert_eq!(format!("{style}"), "\x1b[1;31m");
//!
//! let style: Style = style.effect(Effect::SolidUnderline);
//! assert_eq!(format!("{style}"), "\x1b[1;4;31m");
//!
//! let styled = style.applied_to("Some content");
//! assert_eq!(format!("{styled}"), "\x1b[1;4;31mSome content\x1b[0m");
//! ```
//!
//! [`Styled<C>`] includes a [`Style`] and some content to have the styling applied to. The content can
//! be any type that implements [`Display`](core::fmt::Display). When rendered, the content is preceded by the
//! escape sequence corresponding to the styling, and is succeeded by the escape sequence that resets the styling.
//!
//! A [`Styled<C>`] instance is obtained with the `applied_to()` method available in any [styling element type]
//! and in [`Style`], or with [`Styled<C>::new()`] to create an instance without any styling.
//!
//! ```
//! use fluent_ansi::{prelude::*, Styled};
//!
//! assert_eq!(format!("{}", Effect::Bold.applied_to("Some content")), "\x1b[1mSome content\x1b[0m");
//! assert_eq!(format!("{}", Color::RED.applied_to("Some content")), "\x1b[31mSome content\x1b[0m");
//! assert_eq!(format!("{}", Color::RED.bold().applied_to("Some content")), "\x1b[1;31mSome content\x1b[0m");
//! assert_eq!(format!("{}", Styled::new("Some content").bold().foreground(Color::RED)), "\x1b[1;31mSome content\x1b[0m");
//! ```
//!
//! # Styling methods
//!
//! Since all types are immutable, all methods return a new value, that is a [composed styling type], which is:
//!
//! * [`Styled<C>`] when the method is called from that type,
//! * [`Style`] when the method is called from any other type.
//!
//! There are three group of methods, according to how styling is handled:
//! * [fluent](#fluent-methods) methods
//! * methods for handling styling as [elements and sets](#methods-for-elements-and-sets)
//! * methods for handling styling as [attributes](#methods-for-attributes)
//!
//! <div class="warning">
//!
//!    In the methods docs below, the links take to their implementation in [`Style`], but they are the same for all types.
//!
//! </div>
//!
//! <div class="warning">
//!
//!    Although some methods below are documented with varying signatures (e.g. `color(TargetedColor)` and `color(impl Into<Color>)`),
//!    each method name has a single implementation with a generic argument in each type. Check the linked method documentation to see
//!    the real signature.
//!
//! </div>
//!
//!
//! ## Fluent methods
//!
//! The fluent methods allow to _compose_/_add_/_set_ styling. They are available in all [styling types](#styling-types).
//!
//! | Method | To set what |
//! |--------|-------------|
//! | [`bold()`](Style::bold),<br/>[`italic()`](Style::italic),<br/>[`solid_underline()`](Style::solid_underline),<br/>etc.                                                                                                                                 | effect |
//! | [`effect(impl Into<Effect>)`](Style::effect)                                                                                                                                                                                                          | effect<br/>(including underline effects) |
//! | [`underline_effect(UnderlineEffect)`](Style::underline_effect)                                                                                                                                                                                        | underline effect |
//! | [`foreground(impl Into<Color>)`](Style::foreground)/[`fg(impl Into<Color>)`](Style::fg)<br/>[`background(impl Into<Color>)`](Style::background)/[`bg(impl Into<Color>)`](Style::bg)<br/>[`underline_color(impl Into<Color>)`](Style::underline_color) | color |
//! | [`color(TargetedColor)`](Style::color)                                                                                                                                                                                                                | color |
//! | [`color(impl Into<Color>)`](Style::color)                                                                                                                                                                                                             | foreground color |
//! | [`applied_to(impl Display)`](Style::applied_to) [^applied-to-method]                                                                                                                                                                                  | content |
//!
//!
//! ## Methods for elements and sets
//!
//! All [styling types] can be viewed as _styling sets_ where [styling elements] can be added to or removed from.
//!
//! The `add` method adds an _element_ to a _set_, and the `remove` method removes an _element_ from a _set_.
//!
//!
//! ### The `add` method
//!
//! The `add` method can be used to _compose_/_add_/_set_ some styling, and is available in all [styling types].
//!
//! | Method | To add what |
//! |--------|-------------|
//! | [`add(Effect)`](Style::add)           | effect |
//! | [`add(UnderlineEffect)`](Style::add)  | underline effect |
//! | [`add(TargetedColor)`](Style::add)    | color |
//! | [`add(impl Into<Color>)`](Style::add) | foreground color |
//!
//! ### The `remove` method
//!
//! The `remove` method can be used to _clear_/_remove_ some styling, and is available in the [composed styling types].
//!
//! | Method | To remove what | Note |
//! |--------|----------------|------|
//! | [`remove(Effect)`](Style::remove)          | effect |
//! | [`remove(UnderlineEffect)`](Style::remove) | underline effect | Remove the specific effect, if set |
//! | [`remove(UnderlineStyle)`](Style::remove)  | underline effect | Remove any underline effect that may be set |
//! | [`remove(ColorTarget)`](Style::remove)     | color |
//!
//! ### Example
//!
//! ```
//! use fluent_ansi::{prelude::*, ColorTarget, UnderlineStyle, Styled};
//!
//! let styled = Styled::new("Some content")
//!     .add(Effect::Bold)
//!     .add(Effect::DottedUnderline)
//!     .add(Color::RED.for_underline());
//! assert_eq!(format!("{styled}"), "\x1b[1;4:4;58;5;1mSome content\x1b[0m");
//!
//! // ...
//!
//! let altered_styled = styled
//!     .remove(UnderlineStyle)
//!     .remove(ColorTarget::Underline)
//!     .add(Color::indexed(42).for_background());
//! assert_eq!(format!("{altered_styled}"), "\x1b[1;48;5;42mSome content\x1b[0m");
//! ```
//!
//!
//! ## Methods for attributes
//!
//! Styling can be seen as _attributes_, which have _values_. The type of the _value_ varies according to the _attribute_.
//!
//! | Attribute | Value type | Meaning |
//! |-----------|------------|---------|
//! | [`Effect`]          | [`bool`]                    | Whether the effect is set or not |
//! | [`UnderlineEffect`] | [`bool`]                    | Whether the specific underline effect is set or not |
//! | [`UnderlineStyle`]  | [`Option<UnderlineEffect>`] | Which underline effect is in use, if any |
//! | [`ColorTarget`]     | [`Option<Color>`]           | Which color is in use for that target, if any |
//!
//! The `set` method can be used to _add_/_set_/_clear_/_remove_ some styling, and the `get` method can be used to _query_ any styling.
//! Both methods are available in all [composed styling types].
//!
//! There are also styling-specific variations for the `set` and `get` methods, in addition to the `get_effects()`, that returns an
//! iterator on the effects that are set.
//!
//! | Methods | Styling |
//! |---------|---------|
//! | [`set(Effect, bool)`](Style::set)                            <br/> [`set_effect(Effect, bool)`](Style::set_effect)                                          | effect
//! | [`set(UnderlineEffect, bool)`](Style::set)                   <br/> [`set_effect(UnderlineEffect, bool)`](Style::set_effect)                                 | underline effect |
//! | [`set(UnderlineStyle, Option<UnderlineEffect>)`](Style::set) <br/> [`set_underline_effect(Option<UnderlineEffect>)`](Style::set_underline_effect)           | underline effect |
//! | [`set(ColorTarget, Option<Color>)`](Style::set)              <br/> [`set_color(ColorTarget, Option<impl Into<Color>>)`](Style::set_color) [^set-color-none] | color |
//!
//! | Methods | Styling |
//! |---------|---------|
//! | [`get(Effect) -> bool`](Style::get)                            <br/> [`get_effect(Effect) -> bool`](Style::get_effect) <br/>[`get_effects() -> GetEffects`](Style::get_effect) | effect
//! | [`get(UnderlineEffect) -> bool`](Style::get)                   <br/> [`get_effect(UnderlineEffect) -> bool`](Style::get_effect)                                                | underline effect |
//! | [`get(UnderlineStyle) -> Option<UnderlineEffect>`](Style::get) <br/> [`get_underline_effect() -> Option<UnderlineEffect>`](Style::get_underline_effect)                        | underline effect |
//! | [`get(ColorTarget) -> Option<Color>`](Style::get)              <br/> [`get_color(ColorTarget) -> Option<Color>)`](Style::get_color)                                            | color |
//!
//! ### Example
//!
//! ```
//! use fluent_ansi::{prelude::*, ColorTarget, UnderlineStyle, Styled};
//!
//! let styled = Styled::new("Some content")
//!     .set(Effect::Bold, true)
//!     .set(Effect::DottedUnderline, true)
//!     .set(ColorTarget::Underline, Some(Color::RED.to_color()));
//! assert_eq!(format!("{styled}"), "\x1b[1;4:4;58;5;1mSome content\x1b[0m");
//!
//! // ...
//!
//! let altered_styled = styled
//!     .set(UnderlineStyle, None)
//!     .set(ColorTarget::Underline, None)
//!     .set(ColorTarget::Background, Some(Color::indexed(42).to_color()));
//! assert_eq!(format!("{altered_styled}"), "\x1b[1;48;5;42mSome content\x1b[0m");
//! ```
//!
//!
//! # The [`Reset`] singleton
//!
//! [`Reset`] is a singleton value that represents the "reset" ANSI code. It can be used to manually control
//! the starting and ending escape sequences instead of using the [`Styled<C>`] type with an enclosed content.
//!
//! ```
//! use fluent_ansi::{prelude::*, Reset};
//!
//! let style = Color::RED.bold();
//! let output = format!("{style}Some content{Reset}");
//!
//! assert_eq!(output, "\x1b[1;31mSome content\x1b[0m");
//! ```
//!
//!
//! [styling types]: #styling-types
//! [styling element]: #styling-element-types
//! [styling elements]: #styling-element-types
//! [styling element type]: #styling-element-types
//! [styling element types]: #styling-element-types
//! [composed styling type]: #composed-styling-types
//! [composed styling types]: #composed-styling-types
//!
//! [^applied-to-method]: [`applied_to()`](Style::applied_to) is not available in [`Styled<C>`] values, and always returns a [`Styled<C>`].
//! [^set-color-none]: To clear a color with [`set_color()`](Style::set_color), the color type must be specified in the `None` value as e.g.
//!     `None::<Color>`. As an alternative, use the [`Color::none()`](color::Color::none) method.

pub use crate::{
    effect::*, reset::*, style::*, styled::*, styling_attribute::*, styling_element::*,
    targeted_color::*,
};

mod colors;
mod effect;
mod reset;
mod style;
mod styled;
mod styling_attribute;
mod styling_element;
mod targeted_color;

mod impl_macros;

pub mod color {
    //! Color types and trait.
    //!
    //! There are 4 color types:
    //! - [`BasicColor`]: 3-bit colors with 8 variants.
    //! - [`SimpleColor`]: Adds bright variants to the [`BasicColor`]s, totalling 16 colors.
    //! - [`IndexedColor`]: 8-bit colors (256 colors).
    //! - [`RGBColor`]: RGB colors (24-bit/true color).
    //!
    //! The enum [`Color`] unifies all the color types in a single type and have members to access or create colors of all types:
    //!
    //! ```
    //! use fluent_ansi::{prelude::*, color::{BasicColor, IndexedColor, RGBColor, SimpleColor}};
    //!
    //! assert_eq!(Color::RED, BasicColor::Red);
    //! assert_eq!(Color::RED.bright(), SimpleColor::new_bright(BasicColor::Red));
    //! assert_eq!(Color::indexed(127), IndexedColor(127));
    //! assert_eq!(Color::rgb(0, 128, 255), RGBColor::new(0, 128, 255));
    //! ```
    //!
    //! All color types are convertible to [`Color`] and can be used where an `impl Into<Color>` value is expected:
    //!
    //! ```
    //! use fluent_ansi::{prelude::*, color::{BasicColor, IndexedColor, RGBColor, SimpleColor}, ColorTarget, Style};
    //!
    //! let style = Style::new();
    //!
    //! let _ = style.foreground(BasicColor::Red);
    //! let _ = style.foreground(SimpleColor::new_bright(BasicColor::Red));
    //! let _ = style.foreground(IndexedColor::new(128));
    //! let _ = style.foreground(RGBColor::new(0, 128, 255));
    //!
    //! let _ = style.background(BasicColor::Red);
    //! let _ = style.background(SimpleColor::new_bright(BasicColor::Red));
    //! let _ = style.background(IndexedColor::new(128));
    //! let _ = style.background(RGBColor::new(0, 128, 255));
    //!
    //! let _ = style.set_color(ColorTarget::Foreground, Some(BasicColor::Red));
    //! let _ = style.set_color(ColorTarget::Background, Some(SimpleColor::new_bright(BasicColor::Red)));
    //! let _ = style.set_color(ColorTarget::Foreground, Some(IndexedColor::new(128)));
    //! let _ = style.set_color(ColorTarget::Background, Some(RGBColor::new(0, 128, 255)));
    //! ```

    pub use crate::colors::*;
}

pub mod prelude {
    //! Re-exports the minimal set of items to style some content.
    //!
    //! This module can be imported to have access to the minimal items to build a [`Styled<C>`](crate::Styled) value from
    //! effects and colors.
    //!
    //! ```
    //! use fluent_ansi::prelude::*;
    //!
    //! let styled = Color::RED.for_background().bold().applied_to("Some content");
    //! ```

    pub use crate::Effect;
    pub use crate::UnderlineEffect;
    pub use crate::color::Color;
}
