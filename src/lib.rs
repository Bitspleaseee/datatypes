#![feature(transpose_result)]
#![feature(try_from)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate failure;
extern crate serde_json;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate regex;

pub type Token = u32;

#[macro_use]
pub mod macros;
pub mod admin;
pub mod auth;
pub mod content;
pub mod error;
pub mod payloads;
pub mod valid;
