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

| Type \\ Trait | `Composed` | ? | `Additive` | `ToStyle`<br>`: Into<Style>` | `From<T> for Style` | `StylingElement` |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: |
| `Styled<C>`                                        | X | ? | X                  |                   |                  |                                  |
| `Style`                                            | X | ? | X[^Additive-for-T] | X[^ToStyle-for-T] | X[^T-from-T]     |                                  |
| `Effect`<br/>`UnderlineEffect`<br/>`TargetedColor` |   | ? | X[^Additive-for-T] | X[^ToStyle-for-T] | X[^Style-from-T] | X                                |
| `impl ColorKind`                                   |   | ? | X[^Additive-for-T] | X[^ToStyle-for-T] | X[^Style-from-T] | X[^StylingElement-for-ColorKind] |

| `+` | `impl StylingElement` | `Style` | `Styled<C>` |
| :--- | :---: | :---: | :---: |
| `impl StylingElement` | `-> Style`     | `-> Style`     | `-> Styled<C>` |
| `Style`               | `-> Style`     | `-> Style`     | `-> Styled<C>` |
| `Styled<C>`           | `-> Styled<C>` | `-> Styled<C>` |                |

| `-` | `impl StylingAttribute` |
| :--- | :---: |
| `Style`               | `-> Style`     |
| `Styled<C>`           | `-> Styled<C>` |

| `?=` | `impl StylingElement` | `Style` | `impl StylingAttribute` |
| :--- | :---: | :---: | :---: |
| `Style`               | `+=` | `+=` | `-=` |
| `Styled<C>`           | `+=` | `+=` | `-=` |

[^Additive-for-T]: blanket implementation: `impl<T> Additive for T where T: Into<Style>`
[^StylingElement-for-ColorKind]: Blanket implementation: `impl<T> StylingElement for T where T: ColorKind`
[^Style-from-T]: Blanket implementation: `impl<T> From<T> for Style where T: StylingElement`
[^ToStyle-for-T]: Blanket implementation: `impl<T> ToStyle for T where T: Into<Style>`

## `StylingAttribute`
- `Effect`<br/>`UnderlineEffect`
- `ColorTarget`
- `UnderlineStyle`
