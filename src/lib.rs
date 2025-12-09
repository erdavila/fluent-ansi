#![no_std]
#![warn(clippy::pedantic)]

pub use crate::{color::*, flags::*, format::*, formatted::*};

mod color;
mod flags;
mod format;
mod formatted;
mod private;

#[cfg(test)]
mod tests;
