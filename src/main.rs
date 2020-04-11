//#![feature(alloc_system)]
//use libc::malloc;


//use std::os::raw;
#![feature(optin_builtin_traits)]
#![feature(alloc_error_handler)]
#![feature(custom_attribute, lang_items, panic_info_message, start)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]






//#[macro_use]
//extern crate bsd_std as bsd_lib;
#[macro_use]
extern crate bsd_std as std;

use std::io;

extern crate alloc;

pub mod mutex;
pub mod lang;
pub mod allocator;

use crate::allocator::FreebsdAllocator;


#[global_allocator]
static ALLOCATOR: FreebsdAllocator = FreebsdAllocator;

#[start]
fn start(_argc: isize, _argv:*const *const u8) -> isize {
    println!("Hello, start!");
    main();

    0
}

#[no_mangle]
fn main() {

    println!("Hello, world!");
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
