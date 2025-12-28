# Types and their trait implementations

It includes blanket implementations.

| Type \\ Trait | `StyleSet: ToStyleSet` | `ToStyleSet` | `ToStyle: Into<Style>` | `AppliedTo: ToStyle + ToStyleSet` | `ColorKind: Into<Color>` | `StyleElement: AppliedTo` | `StyleAttribute` |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `Styled` | X | X | | | | | |
| `Style` | X | X | X | X | | | |
| `Flag` | | X | X | X | | X | X |
| `ColorInAPlane` | | X | X | X | | X | |
| `BasicColor`<br/>`SimpleColor`<br/>`EightBitColor`<br/>`RGBColor`<br/>`Color` | | | | | X | | |
| `Reset` | | | X | | | | |
| `Plane` | | | | | | | X |
