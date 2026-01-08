- `const` everywhere - allow defining styles at compile time
  - Complete implementation requires experimental feature (`const_trait_impl`) and nightly compiler
    - Implement what is possible in `main`, and the remaining in a branch.
- `Style` merge:
  ```rust
  impl Style {
      pub fn merge(self, other: Style) -> Self {
          self.merge_style(other)
      }

      pub fn merge_style(self, other: Style) -> Self {
          ...
      }
  }

  impl<C: Display> Styled<C> {
      pub fn merge_style(self, other: Style) -> Self {
          self.modify_style(|style| style.merge_style(other))
      }
  }
  ```
- Trait to apply methods in any content that implements `Display`
  - Example: `"Some content".bold().solid_underline()`
  - with method `with_style(Style)`
- Handle nesting. How?!
  - Ideas:
    - https://crates.io/crates/ansiconst
    - https://doc.rust-lang.org/nightly/core/macro.format_args.html
- Consider implementing the `Add` trait instead of defining the `add` method in fluent types.
