- Revert back to traits instead of macros
  - Is `ToStyle` needed?
    - It seems the same as `ToComposed<Composed = Style>`.
  - Make `Copy` supertrait of all traits.
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
- Handle `Styled` concatenation
  ```rust
  let x = Styled::new("My ");
  let y = Styled::new(2).bold();
  let z = Styled::new(" cats").bold();

  let a = x.concat(y).red().concat(z);
  // or:
  let a = (x + y).red() + z;

  assert_eq!(a.to_string(), "\x1b[31mMy \x1b[1m2\x1b[39m cats\x1b[0m");
  ```
  - Can handle nesting?
- Consider implementing the `Add` and `Sub` traits that delegate to the `add` and `remove` methods.
