# fluent-ansi

## Project Overview

`fluent-ansi` is a Rust library designed to handle ANSI escape sequences for the terminal. It provides a modular, composable, and fluent API for styling text with colors and effects (like bold, italic).

**Key Features:**
*   **no_std Compatible:** Designed to work without the standard library, relying on `core::fmt::Display`.
*   **Fluent API:** Allows method chaining (e.g., `Color::RED.for_fg().bold().applied_to("text")`).
*   **Immutability:** All styling types are immutable and most implement `Copy`.
*   **Type Safety:** style is handled through typed structs like `Style`, `Styled<C>`, `Effect`, and `TargetedColor`.

## Architecture

*   **`src/lib.rs`**: The library entry point. It contains extensive documentation (including doctests), module declarations, and re-exports.
*   **`src/colors/`**: Defines different color types:
    *   `basic.rs`: Basic 3-bit colors.
    *   `simple.rs`: 3-bit/4-bit colors (basic + bright).
    *   `indexed.rs`: 8-bit (256-color) support.
    *   `rgb.rs`: 24-bit (TrueColor/RGB) support.
    *   `color.rs`: Unified `Color` enum.
*   **`src/style.rs`**: Defines the `Style` struct, which is a container for style elements.
*   **`src/style/encoded_effects.rs`**: Internal helper for encoding effects and managing exclusivity (e.g. underline effects).
*   **`src/styled.rs`**: Defines `Styled<C>`, which wraps content `C` with a `Style`.
*   **`src/effect.rs`**: Defines ANSI effects (Bold, Italic, Underline, etc.).
*   **`src/effect/underline.rs`**: Defines `UnderlineEffect` enum for extended underline effects.
*   **`src/styling_element.rs`**: Trait for types that can be added to a style.
*   **`src/styling_attribute.rs`**: Trait for types that can be set, get or unset in a style.
*   **`src/targeted_color.rs`**: Associates a color with a target (Foreground, Background, or Underline).

## Building and Running

This is a standard Rust project using Cargo.

*   **Build:**
    ```bash
    cargo build
    ```

*   **Run Tests:**
    ```bash
    cargo test
    ```

*   **Documentation:**
    To generate and view the documentation:
    ```bash
    cargo doc --no-deps
    ```

## Development Conventions

*   **Code Style:** Follows standard Rust conventions.
*   **Documentation:** The project places a strong emphasis on inline documentation and doctests within `src/lib.rs`. Ensure new features include corresponding documentation and examples.
*   **Testing:** Unit tests are used. `tests/common/mod.rs` provides a helper macro `assert_display!` for verifying styled output.
*   **Pedantic Clippy:** The project uses `#![warn(clippy::pedantic)]`, so ensure code satisfies Clippy's pedantic lints.
