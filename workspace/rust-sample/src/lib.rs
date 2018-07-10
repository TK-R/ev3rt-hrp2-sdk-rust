#![no_std]
#![feature(panic_implementation, lang_items, asm, core_intrinsics)]
#![allow(dead_code, improper_ctypes, unused_imports)]
extern crate rcstring;
use core::intrinsics;
use core::panic::PanicInfo;
use rcstring::*;

mod ev3;
use button::ButtonT;

use ev3::*;
use led::LEDColorT;
use motor::{MotorPortT, MotorTypeT};

#[no_mangle]
pub extern "C" fn main_task(_exinf: i32) {
	let mut buf = [0u8; 64];
	lcd::set_font(lcd::LCDFontT::EV3FontSmall);
	let mut count = 0;
	loop {
		let s: &str = write_to::show(&mut buf, format_args!("a={}", count)).unwrap();
		lcd::draw_string(s, 0, 10);
		count += 1;
		ev3::lap_dly_tsk(100);
	}
}

#[allow(dead_code)]
fn button_led_test() {
	loop {
		ev3::lap_dly_tsk(100);
		if button::is_pressed(ButtonT::RightButton) {
			led::set_led_color(led::LEDColorT::LEDRed);
		} else if button::is_pressed(ButtonT::LeftButton) {
			led::set_led_color(LEDColorT::LEDGreen);
		} else if button::is_pressed(button::ButtonT::UpButton) {
			led::set_led_color(LEDColorT::LEDOrange);
		} else {
			led::set_led_color(LEDColorT::LEFOff);
		}
	}
}

#[allow(dead_code)]
fn button_motor_test() {
	motor::config(MotorPortT::EV3PortA, MotorTypeT::LargeMotor);
	motor::config(MotorPortT::EV3PortB, MotorTypeT::MediumMotor);

	loop {
		ev3::lap_dly_tsk(100);
		if button::is_pressed(ButtonT::RightButton) {
			motor::set_power(MotorPortT::EV3PortA, 50);
		} else if button::is_pressed(ButtonT::LeftButton) {
			motor::set_power(MotorPortT::EV3PortA, -50);
		} else if button::is_pressed(button::ButtonT::UpButton) {
			motor::set_power(MotorPortT::EV3PortB, 50);
		} else if button::is_pressed(button::ButtonT::DownButton) {
			motor::set_power(MotorPortT::EV3PortB, -50);
		} else {
			motor::stop(MotorPortT::EV3PortA, false);
			motor::stop(MotorPortT::EV3PortB, false);
		}
	}
}
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
	unsafe { intrinsics::abort() }
}

// These functions are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}
