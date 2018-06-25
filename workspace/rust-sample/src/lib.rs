#![no_std]
#![feature(panic_implementation, lang_items, asm, core_intrinsics)]
#![allow(dead_code, improper_ctypes)]
use core::intrinsics;
use core::panic::PanicInfo;
mod ev3;
use button::ButtonT;
#[allow(unused_imports)]
use ev3::{battery, button, led, motor};
use led::LEDColorT;
use motor::{MotorPortT, MotorTypeT};

#[no_mangle]
pub extern "C" fn main_task(_exinf: i32) {
	button_motor_test();
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

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
	unsafe { intrinsics::abort() }
}
