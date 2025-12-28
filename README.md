# fluent-ansi

`fluent-ansi` is a Rust library designed to handle ANSI escape sequences for the terminal. It provides a modular, composable, and fluent API for styling text with colors and effects (like bold, italic).

## Key Features

*   **`no_std` Compatible:** Designed to work without the standard library, relying on `core::fmt::Display`.
*   **Fluent API:** Allows method chaining (e.g., `Color::RED.for_fg().bold().applied_to("text")`).
*   **Immutability:** All styling types are immutable and most implement `Copy`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fluent-ansi = "0.2.1"
```

## Usage

The primary way to use `fluent-ansi` is through its fluent API. You can combine colors and effecs to create a `Style`, and then apply it to any type that implements `Display`.

```rust
use fluent_ansi::{prelude::*, Style, Styled};

// Create a style
let style: Style = Color::RED.for_fg().bold();

// Apply it to some content
let styled: Styled<&str> = style.applied_to("Some content");

// Print it directly
println!("{}", styled);

// Or get the string with escape sequences
let content_with_escape_sequences = format!("{}", styled);
assert_eq!(content_with_escape_sequences, "\x1b[1;31mSome content\x1b[0m");
```

### Composable API

There are several ways to reach the same result, depending on your preference:

```rust
use fluent_ansi::{prelude::*, ColorTarget, Style, TargetedColor};

let stl: Style = Style::new().set(Effect::Bold, true).set(ColorTarget::Foreground, Some(Color::RED.to_color()));
let stl: Style = Style::new().set_effect(Effect::Bold, true).set_color(ColorTarget::Foreground, Some(Color::RED));
let stl: Style = Style::new().add(Effect::Bold).add(TargetedColor::new(Color::RED, ColorTarget::Foreground));
let stl: Style = Style::new().effect(Effect::Bold).color(TargetedColor::new(Color::RED, ColorTarget::Foreground));
let stl: Style = Style::new().bold().fg(Color::RED);
let stl: Style = Effect::Bold.fg(Color::RED);
let stl: Style = Color::RED.for_fg().bold();
```

### Styling Elements

#### Effects

Effects can be used on their own, combined with other elements, or applied to content:

```rust
use fluent_ansi::prelude::*;

assert_eq!(format!("{}", Effect::Bold), "\x1b[1m");
assert_eq!(format!("{}", Effect::Bold.applied_to("Some content")), "\x1b[1mSome content\x1b[0m");
```

#### Underline Styles

In addition to standard effects, specific underline styles are supported (and are mutually exclusive):

```rust
use fluent_ansi::prelude::*;

assert_eq!(format!("{}", Effect::CurlyUnderline), "\x1b[4:3m");
assert_eq!(format!("{}", Style::new().dashed_underline()), "\x1b[4:5m");
```

#### Colors

The library supports Basic (3/4-bit), 8-bit (256 colors), and RGB (TrueColor) colors. Colors must be associated with a `ColorTarget`, and
can also be applied to some content.

```rust
use fluent_ansi::prelude::*;

let red_foreground = Color::RED.for_fg();
assert_eq!(format!("{red_foreground}"), "\x1b[31m");
assert_eq!(format!("{}", red_foreground.applied_to("Some content")), "\x1b[31mSome content\x1b[0m");


let blue_background = Color::BLUE.for_bg();
assert_eq!(format!("{blue_background}"), "\x1b[44m");
assert_eq!(format!("{}", blue_background.applied_to("Some content")), "\x1b[44mSome content\x1b[0m");
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
