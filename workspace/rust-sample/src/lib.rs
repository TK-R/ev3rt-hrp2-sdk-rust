#![no_std]
#![feature(panic_implementation, lang_items, asm, core_intrinsics)]
#![allow(dead_code, improper_ctypes)]

use core::intrinsics;
use core::panic::PanicInfo;
mod ev3;
#[allow(unused_imports)]
use ev3::{battery, button, led, motor};

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! print {
	//    ($($arg:tt)*) => (ev3::lap_syslog(format!(format_args!($($arg)*))));
	($($arg:tt)*) => {
		ev3::lap_syslog("")
	};
}

#[no_mangle]
pub extern "C" fn main_task(_exinf: i32) {
	loop {
		ev3::lap_dly_tsk(100);
		if ev3::lap_is_connect() {
			ev3::lap_syslog("");
		}

		button_led_test();
		println!("");
	}
}
#[allow(dead_code)]

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

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
	unsafe { intrinsics::abort() }
}
