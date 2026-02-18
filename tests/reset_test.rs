use fluent_ansi::*;

use common::*;

mod common;

#[test]
fn reset() {
    assert_display!(Reset, "\x1b[0m");
}
