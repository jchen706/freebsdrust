// For `pub use ::core::{intrinsics,raw}` statements
#![feature(core_intrinsics)]
#![feature(raw)]
#![feature(optin_builtin_traits)]
#![feature(custom_attribute, lang_items, panic_info_message)]


#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// We *are* std
#![no_std]
// libstd-style public modules
//pub mod io;
pub mod os;


#[macro_use]
pub mod macros;


pub mod io;




// Re-export modules from libcore
pub use ::core::any;
pub use ::core::cell;
pub use ::core::clone;
pub use ::core::cmp;
pub use ::core::convert;
pub use ::core::default;
pub use ::core::fmt;
pub use ::core::hash;
pub use ::core::iter;
pub use ::core::intrinsics;
pub use ::core::marker;
pub use ::core::mem;
pub use ::core::ops;
pub use ::core::option;
pub use ::core::ptr;
pub use ::core::raw;
pub use ::core::result;
pub use ::core::slice;

// Declarations to make rust-bindgen code work
mod std {
	pub use ::clone;
    pub use ::cmp;
	pub use ::default;
    pub use ::fmt;
    pub use ::hash;
    pub use ::marker;
	pub use ::mem;
	pub use ::os;
	pub use ::option;
    pub use ::slice;
}




//home/john/Documents/freebsdrust/target/debug/build/project1-ff47afed9c4bf9de/out/bindings.rs
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));