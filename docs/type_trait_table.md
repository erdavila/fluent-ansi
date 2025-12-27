# Types and their trait implementations

It includes blanket implementations.

| Type \\ Trait | `StyleSet: ToStyleSet` | `ToStyleSet` | `ToStyle: Into<Style>` | `AppliedTo: ToStyle + ToStyleSet` | `ColorKind: Into<Color>` | `StyleElement: AppliedTo` | `StyleAttribute` |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `Styled` | X | X | | | | | |
| `Style` | X | X | X | X | | | |
| `Effect` | | X | X | X | | X | X |
| `UnderlineStyle` | | X | X | X | | X | X |
| `TargetedColor` | | X | X | X | | X | |
| `BasicColor`<br/>`SimpleColor`<br/>`IndexedColor`<br/>`RGBColor`<br/>`Color` | | X | X | X | X | X | |
| `Reset` | | | X | | | | |
| `ColorTarget` | | | | | | | X |
| `Underline` | | | | | | | X |
