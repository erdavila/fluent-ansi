- `const` everywhere - allow defining styles at compile time
  - Complete implementation requires experimental feature (`const_trait_impl`) and nightly compiler
    - Implement what is possible in `main`, and the remaining in a branch or behind a feature.
- Trait to apply methods in any content that implements `Display`
  - Example: `"Some content".bold().solid_underline()`
  - with method `with_style(Style)`
- Handle nesting. How?!
  - Ideas:
    - https://crates.io/crates/ansiconst
    - https://doc.rust-lang.org/nightly/core/macro.format_args.html
- Consider implementing the `Add` and `Sub` traits that delegate to the `add` and `remove` methods.
- Add a form to enable/disable styling.
  - Idea:
    ```rust
    let style = Style::set_enabled(std::io::stdout().is_terminal());

    println!("{}", style.applied_to("Some content").bold().color(Color::RED));
    ```
