#![cfg_attr(not(test), no_std)]
#![warn(clippy::pedantic)]
//! `yet_another_ansi_lib` is a library to handle ANSI escape sequences for the terminal.
//! It is `no_std`, and relies on the [`Display`](core::fmt::Display) trait to render the sequences.
//!
//! ```
//! use yet_another_ansi_lib::{prelude::*, Format, Formatted};
//!
//! let format: Format = BasicColor::Red.in_fg().bold();
//! let formatted: Formatted<&str> = format.applied_to("Some content");
//!
//! println!("{}", formatted);
//!
//! let content_with_escape_sequences = format!("{}", formatted);
//! assert_eq!(content_with_escape_sequences, "\x1b[1;31mSome content\x1b[0m");
//! ```
//!
//! It has modular and composable values, and with its fluent methods, it provides several forms to
//! reach the same result. For instance, all the lines below result in the same [`Format`] value:
//!
//! ```
//! use yet_another_ansi_lib::{prelude::*, ColorInAPlane, Format, Plane};
//!
//! let fmt: Format = Format::new().set(Flag::Bold, true).set(Plane::Foreground, Some(BasicColor::Red.to_color()));
//! let fmt: Format = Format::new().set_flag(Flag::Bold, true).set_color(Plane::Foreground, Some(BasicColor::Red));
//! let fmt: Format = Format::new().add(Flag::Bold).add(ColorInAPlane::new(BasicColor::Red, Plane::Foreground));
//! let fmt: Format = Format::new().flag(Flag::Bold).color(ColorInAPlane::new(BasicColor::Red, Plane::Foreground));
//! let fmt: Format = Format::new().bold().fg(BasicColor::Red);
//! let fmt: Format = Flag::Bold.fg(BasicColor::Red);
//! let fmt: Format = BasicColor::Red.in_fg().bold();
//! ```
//!
//! All types are immutable and implement [`Copy`], except for [`Formatted<C>`](Formatted),
//! which is copiable only if its content `C` type is also copiable.
//!
//!
//! # [`Format`] and [`Formatted<C>`]
//!
//! [`Format`] is a container for formatting elements.
//!
//! [`Formatted<C>`] includes a [`Format`] and some content to have the formatting applied to. The content can
//! be any type that implements [`Display`](core::fmt::Display). When rendered, the content is preceded by the
//! escape sequence corresponding to the formatting, and is succeeded by the escape sequence that resets the formatting.
//!
//! A [`Formatted<C>`] instance is obtained with the [`applied_to()`](AppliedTo::applied_to) method in any formatting type,
//! or with [`Formatted<C>::new()`] to create an instance without any formatting.
//!
//! ```
//! use yet_another_ansi_lib::{prelude::*, Formatted};
//!
//! let flag = Flag::Bold;
//! let fg_color = BasicColor::Red.in_fg();
//! let format = BasicColor::Red.in_fg().bold();
//!
//! assert_eq!(format!("{}", Flag::Bold.applied_to("Some content")), "\x1b[1mSome content\x1b[0m");
//! assert_eq!(format!("{}", BasicColor::Red.in_fg().applied_to("Some content")), "\x1b[31mSome content\x1b[0m");
//! assert_eq!(format!("{}", BasicColor::Red.in_fg().bold().applied_to("Some content")), "\x1b[1;31mSome content\x1b[0m");
//! assert_eq!(format!("{}", Formatted::new("Some content").bold().fg(BasicColor::Red)), "\x1b[1;31mSome content\x1b[0m");
//! ```
//!
//! # Formatting elements
//!
//! Flags and colors are the formatting elements. They are combined into [`Format`] values, even if not explicitly.
//!
//! Most of the methods are provided by traits that must be imported in order to make the methods available. The [`prelude`]
//! includes those traits, and may be imported too:
//!
//! ```
//! use yet_another_ansi_lib::prelude::*;
//! ```
//!
//! ## Flags
//!
//! Flags can be used on their own, combined with other formatting elements, or applied to some content:
//!
//! ```
//! use yet_another_ansi_lib::prelude::*;
//!
//! assert_eq!(format!("{}", Flag::Bold), "\x1b[1m");
//! assert_eq!(format!("{}", Flag::Bold.fg(BasicColor::Red)), "\x1b[1;31m");
//! assert_eq!(format!("{}", Flag::Bold.applied_to("Some content")), "\x1b[1mSome content\x1b[0m");
//! ```
//!
//! ## Colors
//!
//! There is a handful of color types, which are described in the [`color`] module.
//!
//! Colors by themselves are not useful as formatting. They must be associated to a [`Plane`], which is either foreground or
//! background. The type [`ColorInAPlane`] associates a color with a [`Plane`].
//!
//! ```
//! use yet_another_ansi_lib::{prelude::*, ColorInAPlane, Plane};
//!
//! // Both lines below are equivalent
//! let red_in_foreground: ColorInAPlane = BasicColor::Red.in_fg();
//! let red_in_foreground: ColorInAPlane = BasicColor::Red.in_plane(Plane::Foreground);
//! assert_eq!(format!("{red_in_foreground}"), "\x1b[31m");
//!
//! // Both lines below are equivalent
//! let red_in_background: ColorInAPlane = BasicColor::Red.in_bg();
//! let red_in_background: ColorInAPlane = BasicColor::Red.in_plane(Plane::Background);
//! assert_eq!(format!("{red_in_background}"), "\x1b[41m");
//! ```
//!
//! ## Setting and clearing formatting
//!
//! Since all types are immutable, the formatting methods return a new [`Formatted<C>`] when called from that type,
//! or a new [`Format`] when called from any other type.
//!
//!
//! ### Methods provided by the [`ToFormatSet`] trait
//!
//! The following methods _set_ or _add_ some formatting, and are available in [`Flag`], [`ColorInAPlane`], [`Format`] and [`Formatted<C>`] values.
//!
//! | Method | To set what | Note |
//! |--------|-------------|------|
//! | [`bold()`](ToFormatSet::bold),<br/>[`italic()`](ToFormatSet::italic),<br/>etc.          | flag |
//! | [`flag(Flag)`](ToFormatSet::flag)                                                       | flag |
//! | [`fg(impl Into<Color>)`](ToFormatSet::fg)<br/>[`bg(impl Into<Color>)`](ToFormatSet::bg) | color |
//! | [`color(ColorInAPlane)`](ToFormatSet::color)                                            | color |
//! | [`add(Flag)`](ToFormatSet::add)                                                         | flag | See note below. |
//! | [`add(ColorInAPlane)`](ToFormatSet::add)                                                | color | See note below. |
//!
//! *Note*: there is in fact a single [`add()`](ToFormatSet::add) method that takes an <code>impl [FormatElement]</code> argument.
//!
//!
//! ### Methods provided by the [`FormatSet`] trait
//!
//! The methods below _set_ or _clear_ some formatting, and are available in [`Format`] and [`Formatted<C>`] values.
//!
//! | Method | To set what | Note |
//! |--------|-------------|------|
//! | [`set_flag(Flag, bool)`](FormatSet::set_flag)                        | flag |
//! | [`set_color(Plane, Option<impl Into<Color>>)`](FormatSet::set_color) | color | See note \[1] below. |
//! | [`set(Flag, bool)`](FormatSet::set)                                  | flag | See note \[2] below. |
//! | [`set(Plane, Option<Color>)`](FormatSet::set)                        | color | See note \[2] below. |
//! | [`unset(Flag)`](FormatSet::unset)                                    | flag | See note \[3] below. |
//! | [`unset(Plane)`](FormatSet::unset)                                   | color | See note \[3] below. |
//!
//! *Note* \[1]: to clear a color with [`set_color()`](FormatSet::set_color), the color type must be specified in the `None` value:
//!
//! ```
//! # use yet_another_ansi_lib::{prelude::*, Format, Plane, color::Color};
//! # let format = Format::new();
//! # let plane = Plane::Foreground;
//! format.set_color(plane, None::<Color>);
//! ```
//!
//! *Note* \[2]: there is in fact a single [`set()`](FormatSet::set) method that is based on the [`FormatAttribute`] trait.
//!
//! *Note* \[3]: there is in fact a single [`unset()`](FormatSet::unset) method that is based on the [`FormatAttribute`] trait.
//!
//!
//! ## Getting current formatting
//!
//! All methods are provided by the [`FormatSet`] trait, which is implemented for [`Format`] and [`Formatted<C>`].
//!
//! | Method | To get what | Note |
//! |--------|-------------|------|
//! | [`get_flag(Flag) -> bool`](FormatSet::get_flag)             | flag |
//! | [`get_flags() -> GetFlags`](FormatSet::get_flag)            | flag | Returns an iterator on the flags that are currently set. |
//! | [`get_color(Plane) -> Option<Color>`](FormatSet::get_color) | color |
//! | [`get(Flag) -> bool`](FormatSet::get)                       | flag | See note below. |
//! | [`get(Plane) -> Option<Color>`](FormatSet::get)             | color | See note below. |
//!
//! *Note*: there is in fact a single [`get()`](FormatSet::get) method that is based on the [`FormatAttribute`] trait.
//!
//!
//! # The [`Reset`] singleton
//!
//! [`Reset`] is a singleton value that represents the "reset" ANSI code. It can be used to manually control
//! the starting and ending escape sequences instead of using the [`Formatted<C>`] type with an enclosed content.
//!
//! ```
//! use yet_another_ansi_lib::{prelude::*, Reset};
//!
//! let format = BasicColor::Red.in_fg().bold();
//! let output = format!("{format}Some content{Reset}");
//!
//! assert_eq!(output, "\x1b[1;31mSome content\x1b[0m");
//! ```

pub use crate::{
    applied_to::*, color_in_a_plane::*, flags::*, format::*, format_set::*, formatted::*, reset::*,
    to_format::*, to_format_set::*,
};

mod applied_to;
pub mod color;
mod color_in_a_plane;
mod flags;
mod format;
mod format_set;
mod formatted;
mod reset;
mod to_format;
mod to_format_set;

/// Re-exports the minimal set of items to format some content.
///
/// This module can be imported to have access to the minimal items to build a [`Formatted<C>`] value from
/// flags and colors.
///
/// ```
/// use yet_another_ansi_lib::prelude::*;
///
/// let formatted = BasicColor::Red.in_fg().bold().applied_to("Some content");
/// ```
pub mod prelude {
    pub use crate::color::{BasicColor, ColorKind, EightBitColor, RGBColor};
    pub use crate::{AppliedTo, Flag, FormatSet, ToFormatSet};
}

#[cfg(test)]
mod tests;
