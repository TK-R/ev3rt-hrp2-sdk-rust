#![no_std]
#![feature(lang_items, asm, core_intrinsics)]
#![allow(dead_code, improper_ctypes, unused_imports)]
extern crate rcstring;
use core::intrinsics;
use core::panic::PanicInfo;
use rcstring::*;

pub mod ev3;
pub mod sample;

use ev3::battery::*;
use ev3::button;
use ev3::ev3rt::*;
use ev3::lcd::*;
use ev3::led::*;
use ev3::motor::*;
use ev3::sensor::*;

use sample::*;

// A:尻尾モータ		B:Rモータ	C:Lモータ	D:
// 1:タッチセンサ	2:超音波	3:カラー	4:ジャイロ

#[no_mangle]
pub extern "C" fn main_task(_exinf: i32) {
	set_font(LCDFontT::EV3FontLarge);

	//	button_motor_test();
	//	touch_sensor_test(SensorPort::EV3Port1);

	//	gyro_sample(SensorPort::EV3Port4)
	balancer_sample(
		&MotorPort::EV3PortC,
		&MotorPort::EV3PortB,
		&SensorPort::EV3Port1,
		&SensorPort::EV3Port4,
	);
}

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
	unsafe { intrinsics::abort() }
}
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}
