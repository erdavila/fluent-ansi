//! Traits that are implemented by multiple types in the crate.

pub use color_kind::*;
pub use composed::*;
pub use styling_attribute::*;
pub use styling_element::*;

mod color_kind;
mod composed;
mod styling_attribute;
mod styling_element;
