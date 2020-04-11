#![feature(core_intrinsics)]
#![feature(raw)]
#![feature(optin_builtin_traits)]
#![feature(custom_inner_attributes,custom_attribute, lang_items, panic_info_message, alloc_error_handler)]
#![feature(start)]
#![cfg_attr(not(test), no_std)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

const FILEPATH_CODE: &'static str = "src/os/kernel_sys.rs";


//home/john/Documents/freebsdrust/target/debug/build/project1-ff47afed9c4bf9de/out/bindings.rs
//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
//include!(concat!(env::current_dir(),FILEPATH_CODE);


// We *are* std

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




<<<<<<< Updated upstream
//home/john/Documents/freebsdrust/target/debug/build/project1-ff47afed9c4bf9de/out/bindings.rs
//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
=======
>>>>>>> Stashed changes




//#![feature(alloc_system)]
//use libc::malloc;


//use std::os::raw;







//#[macro_use]
//extern crate bsd_std as bsd_lib;
//#[macro_use]
//extern crate bsd_std as std;



extern crate alloc;

//pub mod mutex;
pub mod lang;
pub mod allocator;

use crate::allocator::FreebsdAllocator;


#[global_allocator]
static ALLOCATOR: FreebsdAllocator = FreebsdAllocator;

#[no_mangle]
#[start]
fn start(_argc: isize, _argv:*const *const u8) -> isize {
<<<<<<< Updated upstream
    println!("Hello, start!");
=======
    //println!("Hello, start!");
>>>>>>> Stashed changes
    //main();

    0
}

<<<<<<< Updated upstream
//#[no_mangle]
//#[start]
//fn main() {

 //    println!("Hello, world!");
//}
=======
// #[no_mangle]
// fn main() {

// //     println!("Hello, world!");
// }
>>>>>>> Stashed changes

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
