#![no_std]
#![warn(clippy::pedantic)]

pub use crate::{
    applied_to::*,
    color::*,
    color::{basic::*, eight_bit::*},
    color_in_a_plane::*,
    flags::*,
    format::*,
    format_set::*,
    formatted::*,
    reset::*,
    to_format::*,
    to_format_set::*,
};

mod applied_to;
mod color;
mod color_in_a_plane;
mod flags;
mod format;
mod format_set;
mod formatted;
mod reset;
mod to_format;
mod to_format_set;

#[cfg(test)]
mod tests;
