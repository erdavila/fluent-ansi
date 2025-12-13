#![no_std]
#![warn(clippy::pedantic)]

pub use crate::{
    color::*, flags::*, format::*, format_set::*, formatted::*, to_format::*, to_format_set::*,
};

mod color;
mod flags;
mod format;
mod format_set;
mod formatted;
mod to_format;
mod to_format_set;

#[cfg(test)]
mod tests;
