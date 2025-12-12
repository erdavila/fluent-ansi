#![no_std]
#![warn(clippy::pedantic)]

pub use crate::{clear::*, color::*, flags::*, format::*, formatted::*, to_format_set::*};

mod clear;
mod color;
mod flags;
mod format;
mod formatted;
mod to_format_set;

#[cfg(test)]
mod tests;
