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
//! A [`Styled<C>`] instance is obtained with the `applied_to()` method available in any styling type,
//! or with [`Styled<C>::new()`] to create an instance without any styling.
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
//! A bunch of methods can be used to compose stylings with colors:
//!
//! ```
//! use fluent_ansi::prelude::*;
//!
//! let style_1 = Effect::Bold.fg(Color::RED);
//! let style_2 = Effect::Bold.color(Color::RED.for_fg());
//! let style_3 = Effect::Bold.add(Color::RED.for_fg());
//! let style_4 = Color::RED.for_fg().add(Effect::Bold);
//!
//! assert_eq!(style_1, style_2);
//! assert_eq!(style_1, style_3);
//! assert_eq!(style_1, style_4);
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
//! Prefer to skip mention the color target when using only the foreground color. But
//! if another color target is being used, be explicit:
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
//! | [`bold()`](ToStyleSet::bold),<br/>[`italic()`](ToStyleSet::italic),<br/>[`underline()`](ToStyleSet::underline),<br/>etc.                                     | effect |
//! | [`effect(impl Into<Effect>)`](ToStyleSet::effect)                                                                                                            | effect<br/>(including underline styles) |
//! | [`underline_style(UnderlineStyle)`](ToStyleSet::underline_style)                                                                                             | underline style |
//! | [`fg(impl Into<Color>)`](ToStyleSet::fg)<br/>[`bg(impl Into<Color>)`](ToStyleSet::bg)<br/>[`underline_color(impl Into<Color>)`](ToStyleSet::underline_color) | color |
//! | [`color(TargetedColor)`](ToStyleSet::color)                                                                                                                  | color |
//! | [`color(impl Into<Color>)`](ToStyleSet::color)                                                                                                               | foreground color | See note \[1] below. |
//! | [`add(Effect)`](ToStyleSet::add)                                                                                                                             | effect | See note below. |
//! | [`add(UnderlineStyle)`](ToStyleSet::add)                                                                                                                     | underline style | See note below. |
//! | [`add(TargetedColor)`](ToStyleSet::add)                                                                                                                      | color | See note below. |
//! | [`add(impl Into<Color>)`](ToStyleSet::add)                                                                                                                   | foreground color | See note \[2] below. |
//!
//! *Note* \[1]: there is in fact a single [`color()`](ToStyleSet::color) method that takes an <code>impl Into\<[TargetedColor]></code> argument.
//!
//! *Note* \[2]: there is in fact a single [`add()`](ToStyleSet::add) method that takes an <code>impl [StyleElement]</code> argument.
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
//! *Note* \[1]: to clear a color with [`set_color()`](StyleSet::set_color), the color type must be specified in the `None` value.
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
//! let style = Color::RED.bold();
//! let output = format!("{style}Some content{Reset}");
//!
//! assert_eq!(output, "\x1b[1;31mSome content\x1b[0m");
//! ```

pub use crate::{
    effect::*, reset::*, style::*, style_set::*, styled::*, targeted_color::*, to_style_set::*,
};

mod applied_to_method;
mod colors;
mod effect;
mod reset;
mod style;
mod style_set;
mod styled;
mod targeted_color;
mod to_style_set;

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

    pub use crate::UnderlineStyle;
    pub use crate::color::{Color, ToColor};
    pub use crate::{Effect, StyleSet, ToStyleSet};
}
