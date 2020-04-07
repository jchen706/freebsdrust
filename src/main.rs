//#![feature(alloc_system)]
use libc::malloc;


fn main() {
	unsafe {
	malloc(3);
};
    println!("Hello, world!");
}
