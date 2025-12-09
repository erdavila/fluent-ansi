#![no_std]
#![warn(clippy::pedantic)]

pub use crate::{color::*, format::*, formatted::*};

mod color;
mod format;
mod formatted;
mod private;

#[cfg(test)]
mod tests;
