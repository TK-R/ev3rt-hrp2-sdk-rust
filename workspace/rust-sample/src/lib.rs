#![feature(lang_items, asm, core_intrinsics)]
#![no_std]

mod ev3;
#[allow(unused_imports)]
use ev3::{battery, button, led};

#[no_mangle]
pub extern "C" fn main_task(_exinf: i32) {
	loop {
		ev3::lap_dly_tsk(100);
		button_led_test();
	}
}

fn button_led_test() {
	ev3::lap_dly_tsk(100);
	if button::lap_button_is_pressed(button::ButtonT::RightButton) {
		led::lap_set_led_color(led::LEDColorT::LEDRed);
	} else if button::lap_button_is_pressed(button::ButtonT::LeftButton) {
		led::lap_set_led_color(led::LEDColorT::LEDGreen);
	} else if button::lap_button_is_pressed(button::ButtonT::UpButton) {
		led::lap_set_led_color(led::LEDColorT::LEDOrange);
	} else {
		led::lap_set_led_color(led::LEDColorT::LEFOff);
	}
}

#[allow(dead_code)]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[allow(dead_code)]
#[lang = "panic_fmt"]
extern "C" fn panic_fmt() {}
