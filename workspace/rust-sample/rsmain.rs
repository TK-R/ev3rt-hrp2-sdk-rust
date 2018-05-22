#![feature(lang_items, asm, core_intrinsics)]
#![crate_type = "staticlib"]
#![no_std]

#[no_mangle]
pub extern "C" fn rsmain(val: u32) -> u32 {
	let u = val + 2;
	u
}
/*
#[no_mangle]
pub extern "C" fn _sbrk() {}
#[no_mangle]
pub extern "C" fn _exit() {}
#[no_mangle]
pub extern "C" fn _kill() {}
#[no_mangle]
pub extern "C" fn _getpid() {}
*/
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() {}
