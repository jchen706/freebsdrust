
//#![allow(warnings, unused_variables, dead_code, improper_ctypes, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::alloc::{GlobalAlloc, Layout};
use core::fmt;
use core::ptr;
//use crate::mutex::Mutex;
use ::os::kernel_malloc;



pub struct FreebsdAllocator;

/// `LocalAlloc` is an analogous trait to the standard library's `GlobalAlloc`,
/// but it takes `&mut self` in `alloc()` and `dealloc()`.
pub trait LocalAlloc {
    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8;
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout);
}



unsafe impl GlobalAlloc for FreebsdAllocator {

    unsafe fn alloc(&self, _layout: Layout)-> *mut u8 {
        ptr::null_mut()

        




    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        panic!("hello")
    }




}

