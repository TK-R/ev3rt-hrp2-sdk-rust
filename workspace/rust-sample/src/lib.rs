#![feature(lang_items, asm, core_intrinsics)]
#![no_std]

mod api;

#[no_mangle]
pub extern "C" fn main_task(_exinf: i32) {
	loop {
		api::lap_dly_tsk(1000);
		api::lap_speaker_play_tone(440, 300);
	}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() {}
