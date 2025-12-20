# fluent-ansi

`fluent-ansi` is a Rust library designed to handle ANSI escape sequences for the terminal. It provides a modular, composable, and fluent API for styling text with colors and formatting flags (like bold, italic).

## Key Features

*   **`no_std` Compatible:** Designed to work without the standard library, relying on `core::fmt::Display`.
*   **Fluent API:** Allows method chaining (e.g., `Color::RED.in_fg().bold().applied_to("text")`).
*   **Immutability:** All formatting types are immutable and most implement `Copy`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fluent-ansi = "0.1"
```

## Usage

The primary way to use `fluent-ansi` is through its fluent API. You can combine colors and flags to create a `Format`, and then apply it to any type that implements `Display`.

```rust
use fluent_ansi::{prelude::*, Format, Formatted};

// Create a format
let format: Format = Color::RED.in_fg().bold();

// Apply it to some content
let formatted: Formatted<&str> = format.applied_to("Some content");

// Print it directly
println!("{}", formatted);

// Or get the string with escape sequences
let content_with_escape_sequences = format!("{}", formatted);
assert_eq!(content_with_escape_sequences, "\x1b[1;31mSome content\x1b[0m");
```

### Composable API

There are several ways to reach the same result, depending on your preference:

```rust
use fluent_ansi::{prelude::*, ColorInAPlane, Format, Plane};

let fmt: Format = Format::new().set(Flag::Bold, true).set(Plane::Foreground, Some(Color::RED.to_color()));
let fmt: Format = Format::new().set_flag(Flag::Bold, true).set_color(Plane::Foreground, Some(Color::RED));
let fmt: Format = Format::new().add(Flag::Bold).add(ColorInAPlane::new(Color::RED, Plane::Foreground));
let fmt: Format = Format::new().flag(Flag::Bold).color(ColorInAPlane::new(Color::RED, Plane::Foreground));
let fmt: Format = Format::new().bold().fg(Color::RED);
let fmt: Format = Flag::Bold.fg(Color::RED);
let fmt: Format = Color::RED.in_fg().bold();
```

### Formatting Elements

#### Flags

Flags can be used on their own, combined with other elements, or applied to content:

```rust
use fluent_ansi::prelude::*;

assert_eq!(format!("{}", Flag::Bold), "\x1b[1m");
assert_eq!(format!("{}", Flag::Bold.applied_to("Some content")), "\x1b[1mSome content\x1b[0m");
```

#### Colors

The library supports Basic (3/4-bit), 8-bit (256 colors), and RGB (TrueColor) colors. Colors must be associated with a `Plane` (Foreground or Background), and
can also be applied to some content.

```rust
use fluent_ansi::prelude::*;

let red_in_foreground = Color::RED.in_fg();
assert_eq!(format!("{red_in_foreground}"), "\x1b[31m");
assert_eq!(format!("{}", red_in_foreground.applied_to("Some content")), "\x1b[31mSome content\x1b[0m");


let blue_in_background = Color::BLUE.in_bg();
assert_eq!(format!("{blue_in_background}"), "\x1b[44m");
assert_eq!(format!("{}", blue_in_background.applied_to("Some content")), "\x1b[44mSome content\x1b[0m");
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
