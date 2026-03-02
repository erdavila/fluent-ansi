- `const` everywhere - allow defining styles at compile time
  - Complete implementation requires experimental feature (`const_trait_impl`) and nightly compiler
    - Implement what is possible in `main`, and the remaining in a branch or behind a feature.
- Methods for foreground colors.
  ```rust
  let styled = "Hello".styled();
  styled.red();
  styled.bright_red();
  styled.indexed_color(i);
  styled.rgb(r, g, b);
  ```
- Handle nesting. How?!
  - Ideas:
    - https://crates.io/crates/ansiconst
    - https://doc.rust-lang.org/nightly/core/macro.format_args.html
  - Handle `Styled` concatenation and nesting
    ```rust
    let x = Styled::new("My ");
    let y = Styled::new(2).bold();
    let z = Styled::new(" cats").bold();

    let a = x.concat(y).red().concat(z);
    // or:
    let a = (x + y).red() + z;

    assert_eq!(a.to_string(), "\x1b[31mMy \x1b[1m2\x1b[39m cats\x1b[0m");
    ```
    - Some mechanism to isolate nested content styling
    ```rust
    let inner = Styled::new(" inner ").fg(Color::RED).bg(Color::WHITE)
      .isolate(ColorTarget::Foreground)
      .isolate(UnderlineStyle);

    let content = Styled::new("before") + inner + Styled::new("after");
    let content = content.underline().fg(Color::Blue).bg(Color::Yellow);

    assert_eq!(
      content.to_string(),
      format!(
        "{}before{} inner {}after{}",
        BLUE_FG_AND_YELLOW_BG_AND_UNDERLINE,
        RED_FG_AND_NO_UNDERLINE,
        BLUE_FG_AND_UNDERLINE,
        RESET
      )
    );

    ```
