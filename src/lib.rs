#![cfg_attr(not(test), no_std)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
//! `fluent-ansi` is a library to handle ANSI escape sequences for the terminal.
//! It is `no_std`, and relies on the [`Display`](core::fmt::Display) trait to render the sequences.
//!
//! ```
//! use fluent_ansi::{prelude::*, Style, Styled};
//!
//! let style: Style = Color::RED.in_fg().bold();
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
//! use fluent_ansi::{prelude::*, ColorInAPlane, Style, Plane};
//!
//! let stl: Style = Style::new().set(Effect::Bold, true).set(Plane::Foreground, Some(Color::RED.to_color()));
//! let stl: Style = Style::new().set_effect(Effect::Bold, true).set_color(Plane::Foreground, Some(Color::RED));
//! let stl: Style = Style::new().add(Effect::Bold).add(ColorInAPlane::new(Color::RED, Plane::Foreground));
//! let stl: Style = Style::new().effect(Effect::Bold).color(ColorInAPlane::new(Color::RED, Plane::Foreground));
//! let stl: Style = Style::new().bold().fg(Color::RED);
//! let stl: Style = Effect::Bold.fg(Color::RED);
//! let stl: Style = Color::RED.in_fg().bold();
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
//! assert_eq!(format!("{}", Color::RED.in_fg().applied_to("Some content")), "\x1b[31mSome content\x1b[0m");
//! assert_eq!(format!("{}", Color::RED.in_fg().bold().applied_to("Some content")), "\x1b[1;31mSome content\x1b[0m");
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
//! ## Colors
//!
//! There is a handful of color types, which are described in the [`color`] module.
//!
//! Colors by themselves are not useful as styles. They must be associated to a [`Plane`], which is either foreground or
//! background. The type [`ColorInAPlane`] associates a color with a [`Plane`].
//!
//! ```
//! use fluent_ansi::{prelude::*, ColorInAPlane, Plane};
//!
//! // Both lines below are equivalent
//! let red_in_foreground: ColorInAPlane = Color::RED.in_fg();
//! let red_in_foreground: ColorInAPlane = Color::RED.in_plane(Plane::Foreground);
//! assert_eq!(format!("{red_in_foreground}"), "\x1b[31m");
//!
//! // Both lines below are equivalent
//! let red_in_background: ColorInAPlane = Color::RED.in_bg();
//! let red_in_background: ColorInAPlane = Color::RED.in_plane(Plane::Background);
//! assert_eq!(format!("{red_in_background}"), "\x1b[41m");
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
//! The following methods _set_ or _add_ some styling, and are available in [`Effect`], [`ColorInAPlane`], [`Style`] and [`Styled<C>`] values.
//!
//! | Method | To set what | Note |
//! |--------|-------------|------|
//! | [`bold()`](ToStyleSet::bold),<br/>[`italic()`](ToStyleSet::italic),<br/>etc.          | effect |
//! | [`effect(Effect)`](ToStyleSet::effect)                                                | effect |
//! | [`fg(impl Into<Color>)`](ToStyleSet::fg)<br/>[`bg(impl Into<Color>)`](ToStyleSet::bg) | color |
//! | [`color(ColorInAPlane)`](ToStyleSet::color)                                           | color |
//! | [`add(Effect)`](ToStyleSet::add)                                                      | effect | See note below. |
//! | [`add(ColorInAPlane)`](ToStyleSet::add)                                               | color | See note below. |
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
//! | [`set_effect(Effect, bool)`](StyleSet::set_effect)                  | effect |
//! | [`set_color(Plane, Option<impl Into<Color>>)`](StyleSet::set_color) | color | See note \[1] below. |
//! | [`set(Effect, bool)`](StyleSet::set)                                | effect | See note \[2] below. |
//! | [`set(Plane, Option<Color>)`](StyleSet::set)                        | color | See note \[2] below. |
//! | [`unset(Effect)`](StyleSet::unset)                                  | effect | See note \[3] below. |
//! | [`unset(Plane)`](StyleSet::unset)                                   | color | See note \[3] below. |
//!
//! *Note* \[1]: to clear a color with [`set_color()`](StyleSet::set_color), the color type must be specified in the `None` value:
//!
//! ```
//! # use fluent_ansi::{prelude::*, Style, Plane, color::Color};
//! # let style = Style::new();
//! # let plane = Plane::Foreground;
//! style.set_color(plane, None::<Color>);
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
//! | [`get_effect(Effect) -> bool`](StyleSet::get_effect)       | effect |
//! | [`get_effects() -> GetEffects`](StyleSet::get_effects)     | effect | Returns an iterator on the effects that are currently set. |
//! | [`get_color(Plane) -> Option<Color>`](StyleSet::get_color) | color |
//! | [`get(Effect) -> bool`](StyleSet::get)                     | effect | See note below. |
//! | [`get(Plane) -> Option<Color>`](StyleSet::get)             | color | See note below. |
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
//! let style = Color::RED.in_fg().bold();
//! let output = format!("{style}Some content{Reset}");
//!
//! assert_eq!(output, "\x1b[1;31mSome content\x1b[0m");
//! ```

pub use crate::{
    applied_to::*, color_in_a_plane::*, effect::*, reset::*, style::*, style_set::*, styled::*,
    to_style::*, to_style_set::*,
};

mod applied_to;
pub mod color;
mod color_in_a_plane;
mod effect;
mod reset;
mod style;
mod style_set;
mod styled;
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
/// let styled = Color::RED.in_fg().bold().applied_to("Some content");
/// ```
pub mod prelude {
    pub use crate::color::{Color, ColorKind};
    pub use crate::{AppliedTo, Effect, StyleSet, ToStyleSet};
}

#[cfg(test)]
mod tests;
