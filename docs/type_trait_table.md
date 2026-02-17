# Types and their implementations

## Color types

| Type \\ impl | `ColorKind`<br>`: Into<Color>` | `From<T> for TargetedColor` | `From<T> for Color` |
| :--- | :---: | :---: | :---: |
| `BasicColor`<br/>`SimpleColor`<br/>`IndexedColor`<br/>`RGBColor` | X[^ColorKind-for-T] | X[^TargetedColor-from-T] | X            |
| `Color`                                                          | X[^ColorKind-for-T] | X[^TargetedColor-from-T] | X[^T-from-T] |

[^T-from-T]: Blanket implementation provided by [`std`](https://doc.rust-lang.org/std/convert/trait.From.html#impl-From%3CT%3E-for-T).
[^TargetedColor-from-T]: Blanket implementation: `impl<T> From<T> for TargetedColor where T: Into<Color>`
[^ColorKind-for-T]: Blanket implementation: `impl<T> ColorKind for T where T: Into<Color>`

## General

| Type \\ Trait | `Composed` | Fluent methods | `applied_to` method | `From<T> for Style` | Color type | `StylingElement` | `StylingAttribute` |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `Styled`          | X | X |   |                  |   |                                  |   |
| `Style`           | X | X | X | X[^T-from-T]     |   |                                  |   |
| `Effect`          |   | X | X | X[^Style-from-T] |   | X                                | X |
| `UnderlineEffect` |   | X | X | X[^Style-from-T] |   | X                                | X |
| `TargetedColor`   |   | X | X | X[^Style-from-T] |   | X                                |   |
| `impl ColorKind`  |   | X | X | X[^Style-from-T] | X | X[^StylingElement-for-ColorKind] |   |
| `Reset`           |   |   |   |                  |   |                                  |   |
| `ColorTarget`     |   |   |   |                  |   |                                  | X |
| `UnderlineStyle`  |   |   |   |                  |   |                                  | X |

Fluent type = Fluent methods + `applied_to` method + `to_style` method

[^StylingElement-for-ColorKind]: Blanket implementation: `impl<T> StylingElement for T where T: ColorKind`
[^Style-from-T]: blanket implementation: `impl<T> From<T> for Style where T: StylingElement`
