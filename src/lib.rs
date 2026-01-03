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
//! let style: Style = Style::new().bold().fg(Color::RED);
//! let style: Style = Effect::Bold.fg(Color::RED);
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
//! * [Elementar styling types](#elementar-styling-types):
//!   * [Effect types](#effect-types):
//!     * [`Effect`]
//!     * [`UnderlineStyle`]
//!   * [Color types](#color-types):
//!     * [`TargetedColor`]
//!     * The color types in [`color`]
//! * [Composed styling types](#composed-styling-types):
//!   * [`Style`]
//!   * [`Styled<C>`]
//!
//!
//! ## Elementar styling types
//!
//! Each elementar styling type represents a single styling.
//! They can be used on their own or -- through their fluent methods -- combined with other style elements, or applied to some content:
//!
//! ```
//! use fluent_ansi::prelude::*;
//!
//! assert_eq!(format!("{}", Effect::Bold), "\x1b[1m");
//! assert_eq!(format!("{}", Effect::Bold.fg(Color::RED)), "\x1b[1;31m");
//! assert_eq!(format!("{}", Effect::Bold.applied_to("Some content")), "\x1b[1mSome content\x1b[0m");
//! ```
//!
//! When composed, they result in the [`Style`] [composed styling type](#composed-styling-types).
//!
//! When applied to some content, they result in the [`Styled<C>`] [composed styling type](#composed-styling-types).
//!
//!
//! ### Effect types
//!
//! An effect is an elementar styling type that may or may not be present. They correspond to the variants in the [`Effect`] enum.
//!
//! A subset of effects correspond to underline styles. They are mutually exclusive, meaning that when
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
//! The [`UnderlineStyle`] enum variants represent the underline effects.
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
//! let red_foreground: TargetedColor = Color::RED.for_fg();
//! assert_eq!(format!("{}", red_foreground.applied_to("Some content")), "\x1b[31mSome content\x1b[0m");
//!
//! let red_background: TargetedColor = Color::RED.for_bg();
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
//! let rendered_2 = format!("{}", Color::RED.for_fg().applied_to("Some content"));
//! assert_eq!(rendered_1, rendered_2);
//! assert_eq!(rendered_1, "\x1b[31mSome content\x1b[0m");
//! ```
//!
//! You can skip mentioning the color target when using it only in the foreground. But
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
//!     .add(Color::RED.for_fg())
//!     .add(Color::indexed(42).for_underline());
//! let rendered = format!("{}", style.applied_to("Some content"));
//! assert_eq!(rendered, "\x1b[1;31;58;5;42mSome content\x1b[0m");
//! ```
//!
//! ## Composed styling types
//!
//! [`Style`] is the result of composing [elementar styling](#elementar-styling-types) values. A [`Style`]
//! can be used on its own or -- through their fluent methods -- compose with other style elements, or applied to some content:
//!
//! ```
//! use fluent_ansi::{prelude::*, Style};
//!
//! let style: Style = Effect::Bold.fg(Color::RED);
//! assert_eq!(format!("{style}"), "\x1b[1;31m");
//!
//! let style: Style = style.effect(Effect::Underline);
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
//! A [`Styled<C>`] instance is obtained with the `applied_to()` method available in any [elementar styling type](#elementar-styling-types)
//! and in [`Style`], or with [`Styled<C>::new()`] to create an instance without any styling.
//!
//! ```
//! use fluent_ansi::{prelude::*, Styled};
//!
//! assert_eq!(format!("{}", Effect::Bold.applied_to("Some content")), "\x1b[1mSome content\x1b[0m");
//! assert_eq!(format!("{}", Color::RED.applied_to("Some content")), "\x1b[31mSome content\x1b[0m");
//! assert_eq!(format!("{}", Color::RED.bold().applied_to("Some content")), "\x1b[1;31mSome content\x1b[0m");
//! assert_eq!(format!("{}", Styled::new("Some content").bold().fg(Color::RED)), "\x1b[1;31mSome content\x1b[0m");
//! ```
//!
//! # Styling methods
//!
//! Since all types are immutable, all methods return a new [`Styled<C>`] when called from that type,
//! or a new [`Style`] when called from any other type.
//!
//!
//! ## Fluent methods
//!
//! The fluent methods allow to _compose_/_add_/_set_ styling. They are available in all [styling types](#styling-types).
//!
//! <div class="warning">
//!
//!    The links below link to their implementation in [`Effect`], but they are the same for all types.
//!
//! </div>
//!
//! | Method | To set what | Note |
//! |--------|-------------|------|
//! | [`bold()`](Effect::bold),<br/>[`italic()`](Effect::italic),<br/>[`underline()`](Effect::underline),<br/>etc.                                     | effect |
//! | [`effect(impl Into<Effect>)`](Effect::effect)                                                                                                    | effect<br/>(including underline styles) |
//! | [`underline_style(UnderlineStyle)`](Effect::underline_style)                                                                                     | underline style |
//! | [`fg(impl Into<Color>)`](Effect::fg)<br/>[`bg(impl Into<Color>)`](Effect::bg)<br/>[`underline_color(impl Into<Color>)`](Effect::underline_color) | color |
//! | [`color(TargetedColor)`](Effect::color)                                                                                                          | color | See note \[1] below. |
//! | [`color(impl Into<Color>)`](Effect::color)                                                                                                       | foreground color | See note \[1] below. |
//! | [`add(Effect)`](Effect::add)                                                                                                                     | effect | See note \[2] below. |
//! | [`add(UnderlineStyle)`](Effect::add)                                                                                                             | underline style | See note \[2] below. |
//! | [`add(TargetedColor)`](Effect::add)                                                                                                              | color | See note \[2] below. |
//! | [`add(impl Into<Color>)`](Effect::add)                                                                                                           | foreground color | See note \[2] below. |
//! | [`applied_to(impl Display)`](Effect::applied_to)                                                                                                 | content | See note \[3] below. |
//!
//! *Note* \[1]: each styling type has in fact a single [`color()`](Effect::color) method that takes an <code>impl Into\<[TargetedColor]></code> argument.
//!
//! *Note* \[2]: each styling type has in fact a single [`add()`](Effect::add) method that takes an <code>impl [StyleElement]</code> argument.
//!
//! *Note* \[3]: [`applied_to()`](Effect::applied_to) is not available in [`Styled<C>`] values, and always returns a [`Styled<C>`].
//!
//!
//! ## General methods
//!
//! The general methods allow to [modify](#modification-methods) (_compose_/_add_/_set_ and _clear_/_remove_) and [query](#querying-methods)
//! the current styling in [composed styling types](#composed-styling-types).
//!
//!
//! ### Modification methods
//!
//! These methods always return the same type from where they are called.
//!
//! <div class="warning">
//!
//!    The links below link to their implementation in [`Style`], but they are the same for [`Styled<C>`].
//!
//! </div>
//!
//! | Method | To modify what | Note |
//! |--------|----------------|------|
//! | [`set_effect(impl Into<Effect>, bool)`](Style::set_effect)                  | effect (including underline styles) |
//! | [`set_underline_style(Option<UnderlineStyle>)`](Style::set_underline_style) | underline style |
//! | [`set_color(ColorTarget, Option<impl Into<Color>>)`](Style::set_color)      | color | See note \[1] below. |
//! | [`set(Effect, bool)`](Style::set)                                           | effect | See note \[2] below. |
//! | [`set(UnderlineStyle, bool)`](Style::set)                                   | underline style | See note \[2] below. |
//! | [`set(Underline, Option<UnderlineStyle>)`](Style::set)                      | underline style | See note \[2] below. |
//! | [`set(ColorTarget, Option<Color>)`](Style::set)                             | color | See note \[2] below. |
//! | [`unset(Effect)`](Style::unset)                                             | effect | See note \[3] below. |
//! | [`unset(UnderlineStyle)`](Style::unset)                                     | underline style | See note \[3] below. |
//! | [`unset(Underline)`](Style::unset)                                          | underline style | See note \[3] below. |
//! | [`unset(ColorTarget)`](Style::unset)                                        | color | See note \[3] below. |
//!
//! *Note* \[1]: to clear a color with [`set_color()`](Style::set_color), the color type must be specified in the `None` value.
//! To help with that, the [`Color::none()`](color::Color::none) method can be used:
//!
//! ```
//! # use fluent_ansi::{prelude::*, ColorTarget, Style, color::Color};
//! # let style = Style::new();
//! # let color_target = ColorTarget::Foreground;
//! style.set_color(color_target, None::<Color>);
//! // or
//! style.set_color(color_target, Color::none());
//! ```
//!
//! *Note* \[2]: there is in fact a single [`set()`](Style::set) method that is based on the [`StyleAttribute`] trait.
//!
//! *Note* \[3]: there is in fact a single [`unset()`](Style::unset) method that is based on the [`StyleAttribute`] trait.
//!
//!
//! ### Querying methods
//!
//! These methods for querying effects and colors and whose return type depends on what is being queried.
//!
//! <div class="warning">
//!
//!    The links below link to their implementation in [`Style`], but they are the same for [`Styled<C>`].
//!
//! </div>
//!
//! | Method | To query what | Note |
//! |--------|---------------|------|
//! | [`get_effect(impl Into<Effect>) -> bool`](Style::get_effect)                    | effect (including underline styles) |
//! | [`get_underline_style() -> Option<UnderlineStyle>`](Style::get_underline_style) | underline style |
//! | [`get_effects() -> GetEffects`](Style::get_effects)                             | effect | Returns an iterator on the effects that are currently set. |
//! | [`get_color(ColorTarget) -> Option<Color>`](Style::get_color)                   | color |
//! | [`get(Effect) -> bool`](Style::get)                                             | effect | See note below. |
//! | [`get(UnderlineStyle) -> bool`](Style::get)                                     | underline style | See note below. |
//! | [`get(Underline) -> Option<UnderlineStyle>`](Style::get)                        | underline style | See note below. |
//! | [`get(ColorTarget) -> Option<Color>`](Style::get)                               | color | See note below. |
//!
//! *Note*: there is in fact a single [`get()`](Style::get) method that is based on the [`StyleAttribute`] trait.
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

pub use crate::{
    effect::*, reset::*, style::*, style_attribute::*, style_element::*, styled::*,
    targeted_color::*,
};

mod colors;
mod effect;
mod reset;
mod style;
mod style_attribute;
mod style_element;
mod styled;
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
    //! let _ = style.fg(BasicColor::Red);
    //! let _ = style.fg(SimpleColor::new_bright(BasicColor::Red));
    //! let _ = style.fg(IndexedColor::new(128));
    //! let _ = style.fg(RGBColor::new(0, 128, 255));
    //!
    //! let _ = style.bg(BasicColor::Red);
    //! let _ = style.bg(SimpleColor::new_bright(BasicColor::Red));
    //! let _ = style.bg(IndexedColor::new(128));
    //! let _ = style.bg(RGBColor::new(0, 128, 255));
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
    //! let styled = Color::RED.for_bg().bold().applied_to("Some content");
    //! ```

    pub use crate::Effect;
    pub use crate::UnderlineStyle;
    pub use crate::color::Color;
}
