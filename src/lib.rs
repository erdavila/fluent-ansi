#![cfg_attr(not(test), no_std)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
//! `fluent-ansi` is a library to handle ANSI escape sequences for the terminal.
//! It is `no_std`, and relies on the [`Display`](core::fmt::Display) trait to render the sequences.
//!
//! ```
//! use fluent_ansi::{prelude::*, Style, Styled};
//!
//! let style: Style = Color::RED.for_fg().bold();
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
//! let stl: Style = Style::new().set(Effect::Bold, true).set(ColorTarget::Foreground, Some(Color::RED.to_color()));
//! let stl: Style = Style::new().set_effect(Effect::Bold, true).set_color(ColorTarget::Foreground, Some(Color::RED));
//! let stl: Style = Style::new().add(Effect::Bold).add(TargetedColor::new(Color::RED, ColorTarget::Foreground));
//! let stl: Style = Style::new().effect(Effect::Bold).color(TargetedColor::new(Color::RED, ColorTarget::Foreground));
//! let stl: Style = Style::new().bold().fg(Color::RED);
//! let stl: Style = Effect::Bold.fg(Color::RED);
//! let stl: Style = Color::RED.for_fg().bold();
//! ```
//!
//! All types are immutable and implement [`Copy`], except for [`Styled<C>`](Styled),
//! which is copiable only if its content `C` type is also copiable.
//!
//!
//! # [`Style`] and [`Styled<C>`]
//!
//! [`Style`] is a container for styling elements.
//!
//! [`Styled<C>`] includes a [`Style`] and some content to have the styling applied to. The content can
//! be any type that implements [`Display`](core::fmt::Display). When rendered, the content is preceded by the
//! escape sequence corresponding to the styling, and is succeeded by the escape sequence that resets the styling.
//!
//! A [`Styled<C>`] instance is obtained with the [`applied_to()`](AppliedTo::applied_to) method in any styling type,
//! or with [`Styled<C>::new()`] to create an instance without any styling.
//!
//! ```
//! use fluent_ansi::{prelude::*, Styled};
//!
//! assert_eq!(format!("{}", Effect::Bold.applied_to("Some content")), "\x1b[1mSome content\x1b[0m");
//! assert_eq!(format!("{}", Color::RED.for_fg().applied_to("Some content")), "\x1b[31mSome content\x1b[0m");
//! assert_eq!(format!("{}", Color::RED.for_fg().bold().applied_to("Some content")), "\x1b[1;31mSome content\x1b[0m");
//! assert_eq!(format!("{}", Styled::new("Some content").bold().fg(Color::RED)), "\x1b[1;31mSome content\x1b[0m");
//! ```
//!
//! # Style elements
//!
//! Effects and colors are the style elements. They are combined into [`Style`] values, even if not explicitly.
//!
//! Most of the methods are provided by traits that must be imported in order to make the methods available. The [`prelude`]
//! includes those traits, and may be imported too:
//!
//! ```
//! use fluent_ansi::prelude::*;
//! ```
//!
//! ## Effects
//!
//! Effects can be used on their own, combined with other style elements, or applied to some content:
//!
//! ```
//! use fluent_ansi::prelude::*;
//!
//! assert_eq!(format!("{}", Effect::Bold), "\x1b[1m");
//! assert_eq!(format!("{}", Effect::Bold.fg(Color::RED)), "\x1b[1;31m");
//! assert_eq!(format!("{}", Effect::Bold.applied_to("Some content")), "\x1b[1mSome content\x1b[0m");
//! ```
//!
//!
//! ### Underline effects
//!
//! A subset of effects correspond to underline styles. They are mutually exclusive, meaning that when
//! of them is set, any previously set effect is cleared.
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
//! ## Colors
//!
//! There is a handful of color types, which are described in the [`color`] module.
//!
//! Colors by themselves are not useful as styles. They must be associated to a [`ColorTarget`].
//! The type [`TargetedColor`] associates a color with a [`ColorTarget`].
//!
//! ```
//! use fluent_ansi::{prelude::*, ColorTarget, TargetedColor};
//!
//! // Both lines below are equivalent
//! let red_foreground: TargetedColor = Color::RED.for_fg();
//! let red_foreground: TargetedColor = Color::RED.for_target(ColorTarget::Foreground);
//! assert_eq!(format!("{red_foreground}"), "\x1b[31m");
//!
//! // Both lines below are equivalent
//! let red_background: TargetedColor = Color::RED.for_bg();
//! let red_background: TargetedColor = Color::RED.for_target(ColorTarget::Background);
//! assert_eq!(format!("{red_background}"), "\x1b[41m");
//! ```
//!
//! ## Setting and clearing styles
//!
//! Since all types are immutable, the styling methods return a new [`Styled<C>`] when called from that type,
//! or a new [`Style`] when called from any other type.
//!
//!
//! ### Methods provided by the [`ToStyleSet`] trait
//!
//! The following methods _set_ or _add_ some styling, and are available in [`Effect`], [`UnderlineStyle`], [`TargetedColor`], [`Style`] and [`Styled<C>`] values.
//!
//! | Method | To set what | Note |
//! |--------|-------------|------|
//! | [`bold()`](ToStyleSet::bold),<br/>[`italic()`](ToStyleSet::italic),<br/>[`underline()`](ToStyleSet::underline),<br/>etc. | effect |
//! | [`effect(impl Into<Effect>)`](ToStyleSet::effect)                                                                        | effect<br/>(including underline styles) |
//! | [`underline_style(UnderlineStyle)`](ToStyleSet::underline_style)                                                         | underline style |
//! | [`fg(impl Into<Color>)`](ToStyleSet::fg)<br/>[`bg(impl Into<Color>)`](ToStyleSet::bg)                                    | color |
//! | [`color(TargetedColor)`](ToStyleSet::color)                                                                              | color |
//! | [`add(Effect)`](ToStyleSet::add)                                                                                         | effect | See note below. |
//! | [`add(UnderlineStyle)`](ToStyleSet::add)                                                                                 | underline style | See note below. |
//! | [`add(TargetedColor)`](ToStyleSet::add)                                                                                  | color | See note below. |
//!
//! *Note*: there is in fact a single [`add()`](ToStyleSet::add) method that takes an <code>impl [StyleElement]</code> argument.
//!
//!
//! ### Methods provided by the [`StyleSet`] trait
//!
//! The methods below _set_ or _clear_ some styling, and are available in [`Style`] and [`Styled<C>`] values.
//!
//! | Method | To set what | Note |
//! |--------|-------------|------|
//! | [`set_effect(impl Into<Effect>, bool)`](StyleSet::set_effect)                  | effect (including underline styles) |
//! | [`set_underline_style(Option<UnderlineStyle>)`](StyleSet::set_underline_style) | underline style |
//! | [`set_color(ColorTarget, Option<impl Into<Color>>)`](StyleSet::set_color)      | color | See note \[1] below. |
//! | [`set(Effect, bool)`](StyleSet::set)                                           | effect | See note \[2] below. |
//! | [`set(UnderlineStyle, bool)`](StyleSet::set)                                   | underline style | See note \[2] below. |
//! | [`set(Underline, Option<UnderlineStyle>)`](StyleSet::set)                      | underline style | See note \[2] below. |
//! | [`set(ColorTarget, Option<Color>)`](StyleSet::set)                             | color | See note \[2] below. |
//! | [`unset(Effect)`](StyleSet::unset)                                             | effect | See note \[3] below. |
//! | [`unset(UnderlineStyle)`](StyleSet::unset)                                     | underline style | See note \[3] below. |
//! | [`unset(Underline)`](StyleSet::unset)                                          | underline style | See note \[3] below. |
//! | [`unset(ColorTarget)`](StyleSet::unset)                                        | color | See note \[3] below. |
//!
//! *Note* \[1]: to clear a color with [`set_color()`](StyleSet::set_color), the color type must be specified in the `None` value:
//!
//! ```
//! # use fluent_ansi::{prelude::*, ColorTarget, Style, color::Color};
//! # let style = Style::new();
//! # let color_target = ColorTarget::Foreground;
//! style.set_color(color_target, None::<Color>);
//! ```
//!
//! *Note* \[2]: there is in fact a single [`set()`](StyleSet::set) method that is based on the [`StyleAttribute`] trait.
//!
//! *Note* \[3]: there is in fact a single [`unset()`](StyleSet::unset) method that is based on the [`StyleAttribute`] trait.
//!
//!
//! ## Getting current styles
//!
//! All methods are provided by the [`StyleSet`] trait, which is implemented for [`Style`] and [`Styled<C>`].
//!
//! | Method | To get what | Note |
//! |--------|-------------|------|
//! | [`get_effect(impl Into<Effect>) -> bool`](StyleSet::get_effect)                    | effect (including underline styles) |
//! | [`get_underline_style() -> Option<UnderlineStyle>`](StyleSet::get_underline_style) | underline style |
//! | [`get_effects() -> GetEffects`](StyleSet::get_effects)                             | effect | Returns an iterator on the effects that are currently set. |
//! | [`get_color(ColorTarget) -> Option<Color>`](StyleSet::get_color)                   | color |
//! | [`get(Effect) -> bool`](StyleSet::get)                                             | effect | See note below. |
//! | [`get(UnderlineStyle) -> bool`](StyleSet::get)                                     | underline style | See note below. |
//! | [`get(Underline) -> Option<UnderlineStyle>`](StyleSet::get)                        | underline style | See note below. |
//! | [`get(ColorTarget) -> Option<Color>`](StyleSet::get)                               | color | See note below. |
//!
//! *Note*: there is in fact a single [`get()`](StyleSet::get) method that is based on the [`StyleAttribute`] trait.
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
//! let style = Color::RED.for_fg().bold();
//! let output = format!("{style}Some content{Reset}");
//!
//! assert_eq!(output, "\x1b[1;31mSome content\x1b[0m");
//! ```

pub use crate::{
    applied_to::*, effect::*, reset::*, style::*, style_set::*, styled::*, targeted_color::*,
    to_style::*, to_style_set::*,
};

mod applied_to;
pub mod color;
mod effect;
mod reset;
mod style;
mod style_set;
mod styled;
mod targeted_color;
mod to_style;
mod to_style_set;

/// Re-exports the minimal set of items to style some content.
///
/// This module can be imported to have access to the minimal items to build a [`Styled<C>`] value from
/// effects and colors.
///
/// ```
/// use fluent_ansi::prelude::*;
///
/// let styled = Color::RED.for_fg().bold().applied_to("Some content");
/// ```
pub mod prelude {
    pub use crate::UnderlineStyle;
    pub use crate::color::{Color, ColorKind};
    pub use crate::{AppliedTo, Effect, StyleSet, ToStyleSet};
}

#[cfg(test)]
mod tests;
