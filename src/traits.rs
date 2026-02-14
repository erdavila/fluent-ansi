//! Traits that are implemented by multiple types in the crate.

pub use additive::*;
pub use color_kind::*;
pub use composed::*;
pub use styling_attribute::*;
pub use styling_element::*;
pub use to_style::*;
pub use to_styled::*;

mod additive;
mod color_kind;
mod composed;
mod styling_attribute;
mod styling_element;
mod to_style;
mod to_styled;
