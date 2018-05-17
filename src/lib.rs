#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#[cfg_attr(feature = "clippy", allow(never_loop))]

pub mod readline;
pub mod commands;
pub mod parser;
pub mod process;
pub mod token;

extern crate nix;
