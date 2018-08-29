#![no_std]
#![feature(panic_handler, lang_items, asm, core_intrinsics)]
#![allow(dead_code, improper_ctypes, unused_imports)]
extern crate rcstring;
use core::intrinsics;
use core::panic::PanicInfo;
use rcstring::*;

mod ev3;
use button::*;
use battery::*;
use lcd::*;
use ev3::*;
use led::*;
use motor::*;
use sensor::*;

#[no_mangle]
pub extern "C" fn main_task(_exinf: i32) {
	lcd::set_font(LCDFontT::EV3FontLarge);

//	button_motor_test();
//	touch_sensor_test(SensorPort::EV3Port1);

//	color_sensor_reflect_test(SensorPort::EV3Port2);
	color_sensor_raw_test(SensorPort::EV3Port2);
}


/// カラーセンサの反射光の強さをLCDに出力するテスト
fn color_sensor_reflect_test(color_sensor_port:SensorPort) {
	sensor::config(&color_sensor_port, SensorType::ColorSensor);

	loop {
		let reflect = sensor::color_sensor_get_reflect(&color_sensor_port);
		lcd::clear(LCDColorT::EV3LCDWhite);
		lcd::draw_value("Reflect\0", reflect as i32, "%\0", 0, 0 );
		ev3::lap_dly_tsk(100);

	}
}

/// カラーセンサのRGB生値のをLCDに出力するテスト
fn color_sensor_raw_test(color_sensor_port:SensorPort) {
	sensor::config(&color_sensor_port, SensorType::ColorSensor);
	let mut rgb = RGBRaw { red: 0, green: 0,  blue:0 };
	loop {
		sensor::color_sensor_get_rgb_raw(&color_sensor_port, &mut rgb);
		lcd::clear(LCDColorT::EV3LCDWhite);
		lcd::draw_value("Red  \0", rgb.red as i32, "\0", 0, 0 );
		lcd::draw_value("Green\0", rgb.green as i32, "\0", 0, 15 );
		lcd::draw_value("Blue \0", rgb.blue as i32, "\0", 0, 30 );
		
		
		ev3::lap_dly_tsk(100);

	}
}

/// ボタンの押下状態をLCDに出力するテスト
#[allow(dead_code)]
fn touch_sensor_test(touch_sensor_port:SensorPort) {
	sensor::config(&touch_sensor_port, SensorType::TouchSensor);

	loop {
		let pressed = sensor::touch_sensor_is_pressed(&touch_sensor_port);
		lcd::clear(LCDColorT::EV3LCDWhite);
		lcd::draw_value("Touch\0", pressed as i32, "-\0", 0, 0);
		ev3::lap_dly_tsk(100);
	}
}

/// 本体ボタンの押下状態とLEDの動作をテストする
#[allow(dead_code)]
fn button_led_test() {
	loop {
		ev3::lap_dly_tsk(100);
		if button::is_pressed(&ButtonT::RightButton) {
			led::set_led_color(&LEDColorT::LEDRed);
		} else if button::is_pressed(&ButtonT::LeftButton) {
			led::set_led_color(&LEDColorT::LEDGreen);
		} else if button::is_pressed(&ButtonT::UpButton) {
			led::set_led_color(&LEDColorT::LEDOrange);
		} else {
			led::set_led_color(&LEDColorT::LEFOff);
		}
	}
}

/// バッテリ電圧と電流の状態をモニタに出力する
#[allow(dead_code)]
fn battery_test() {
	lcd::set_font(LCDFontT::EV3FontLarge);

	loop {
		ev3::lap_dly_tsk(100);
		
		lcd::clear(LCDColorT::EV3LCDWhite);
		lcd::draw_value("Volt\0", lap_battery_voltage_mv(),"V\0", 0, 0);
		lcd::draw_value("Curr\0", lap_battery_current_ma(),"mA\0", 0, 15);
	}
}

/// 本体ボタンの押下状態とモータの動作をテストする
#[allow(dead_code)]
fn button_motor_test() {
	motor::config(&MotorPortT::EV3PortA, &MotorTypeT::LargeMotor);
	motor::config(&MotorPortT::EV3PortB, &MotorTypeT::MediumMotor);

	loop {
		ev3::lap_dly_tsk(100);
		
		if button::is_pressed(&ButtonT::RightButton) {
			motor::set_power(&MotorPortT::EV3PortA, 50);
		} else if button::is_pressed(&ButtonT::LeftButton) {
			motor::set_power(&MotorPortT::EV3PortA, -50);
		} else if button::is_pressed(&ButtonT::UpButton) {
			motor::set_power(&MotorPortT::EV3PortB, 50);
		} else if button::is_pressed(&ButtonT::DownButton) {
			motor::set_power(&MotorPortT::EV3PortB, -50);
		} else {
			motor::stop(&MotorPortT::EV3PortA, false);
			motor::stop(&MotorPortT::EV3PortB, false);
		}
	}
}

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
	unsafe { intrinsics::abort() }
}
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}
