use fluent_ansi::{Style, Styled, prelude::*};

#[test]
fn styled() {
    let styled: Styled<&str> = "hello".styled();

    assert_eq!(styled.get_content(), &"hello");
    assert_eq!(styled.get_style(), Style::new());
}

#[test]
fn with_style() {
    let style = Style::new().bold().color(Color::RED);

    let styled: Styled<&str> = "hello".with_style(style);

    assert_eq!(styled.get_content(), &"hello");
    assert_eq!(styled.get_style(), style);
}
