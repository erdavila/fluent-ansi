# Types and their implementations

## Color types

| Type \\ impl | `From<T> for TargetedColor` | `From<T> for Color` |
| :--- | :---: | :---: |
| `BasicColor`<br/>`SimpleColor`<br/>`IndexedColor`<br/>`RGBColor` | X[^TargetedColor-from-T] | X                    |
| `Color`                                                          | X[^TargetedColor-from-T] | X[^Color-from-Color] |

[^Color-from-Color]: Blanket implementation provided by [`std`](https://doc.rust-lang.org/std/convert/trait.From.html#impl-From%3CT%3E-for-T).
[^TargetedColor-from-T]: Blanket implementation: `impl<T> From<T> for TargetedColor where T: Into<Color>`

## General

| Type \\ Trait | Composed styling methods | Fluent methods | `applied_to` method | `to_style` method | Color type | `StyleElement` | `StyleAttribute` |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `Styled` | X | X | | | | | |
| `Style` | X | X | X | X | | | |
| `Effect` | | X | X | X | | X | X |
| `UnderlineEffect` | | X | X | X | | X | X |
| `TargetedColor` | | X | X | X | | X | |
| `BasicColor`<br/>`SimpleColor`<br/>`IndexedColor`<br/>`RGBColor`<br/>`Color` | | X | X | X | X | X | |
| `Reset` | | | | X | | | |
| `ColorTarget` | | | | | | | X |
| `UnderlineStyle` | | | | | | | X |

Fluent type = Fluent methods + `applied_to` method + `to_style` method
