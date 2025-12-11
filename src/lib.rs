#![no_std]
#![warn(clippy::pedantic)]

pub use crate::{add::*, clear::*, color::*, flags::*, format::*, formatted::*};

mod add;
mod clear;
mod color;
mod flags;
mod format;
mod formatted;
mod private;

#[cfg(test)]
mod tests;
