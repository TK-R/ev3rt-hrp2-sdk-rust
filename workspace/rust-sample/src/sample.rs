use ev3::battery::*;
use ev3::button::*;
use ev3::ev3rt::*;
use ev3::lcd::*;
use ev3::led::*;
use ev3::motor::*;
use ev3::sensor::*;

/// カラーセンサの反射光の強さをLCDに出力するテスト
pub fn color_sensor_reflect_sample(color_sensor_port: SensorPort) {
	sensor_config(&color_sensor_port, SensorType::ColorSensor);

	loop {
		let reflect = color_sensor_get_reflect(&color_sensor_port);
		lcd_clear(LCDColorT::EV3LCDWhite);
		draw_value("Reflect\0", reflect as i32, "%\0", 0, 0);
		lap_dly_tsk(100);
	}
}

/// カラーセンサのRGB生値のをLCDに出力するサンプル
pub fn color_sensor_raw_sample(color_sensor_port: SensorPort) {
	sensor_config(&color_sensor_port, SensorType::ColorSensor);
	let mut rgb = RGBRaw {
		red: 0,
		green: 0,
		blue: 0,
	};
	loop {
		color_sensor_get_rgb_raw(&color_sensor_port, &mut rgb);
		lcd_clear(LCDColorT::EV3LCDWhite);
		draw_value("Red  \0", rgb.red as i32, "\0", 0, 0);
		draw_value("Green\0", rgb.green as i32, "\0", 0, 15);
		draw_value("Blue \0", rgb.blue as i32, "\0", 0, 30);

		lap_dly_tsk(100);
	}
}

/// ボタンの押下状態をLCDに出力するテスト
#[allow(dead_code)]
pub fn touch_sensor_sample(touch_sensor_port: SensorPort) {
	sensor_config(&touch_sensor_port, SensorType::TouchSensor);

	loop {
		let pressed = touch_sensor_is_pressed(&touch_sensor_port);
		lcd_clear(LCDColorT::EV3LCDWhite);
		draw_value("Touch\0", pressed as i32, "-\0", 0, 0);
		lap_dly_tsk(100);
	}
}

/// 本体ボタンの押下状態とLEDの動作をテストする
#[allow(dead_code)]
pub fn button_led_sample() {
	loop {
		lap_dly_tsk(100);
		if is_pressed(&ButtonT::RightButton) {
			set_led_color(&LEDColorT::LEDRed);
		} else if is_pressed(&ButtonT::LeftButton) {
			set_led_color(&LEDColorT::LEDGreen);
		} else if is_pressed(&ButtonT::UpButton) {
			set_led_color(&LEDColorT::LEDOrange);
		} else {
			set_led_color(&LEDColorT::LEFOff);
		}
	}
}

/// バッテリ電圧と電流の状態をモニタに出力する
#[allow(dead_code)]
pub fn battery_sample() {
	set_font(LCDFontT::EV3FontLarge);

	loop {
		lap_dly_tsk(100);

		lcd_clear(LCDColorT::EV3LCDWhite);
		draw_value("Volt\0", lap_battery_voltage_mv(), "V\0", 0, 0);
		draw_value("Curr\0", lap_battery_current_ma(), "mA\0", 0, 15);
	}
}

/// 本体ボタンの押下状態とモータの動作をテストする
#[allow(dead_code)]
pub fn button_motor_sample() {
	config(&MotorPortT::EV3PortA, &MotorTypeT::LargeMotor);
	config(&MotorPortT::EV3PortB, &MotorTypeT::MediumMotor);

	loop {
		lap_dly_tsk(100);

		if is_pressed(&ButtonT::RightButton) {
			set_power(&MotorPortT::EV3PortA, 50);
		} else if is_pressed(&ButtonT::LeftButton) {
			set_power(&MotorPortT::EV3PortA, -50);
		} else if is_pressed(&ButtonT::UpButton) {
			set_power(&MotorPortT::EV3PortB, 50);
		} else if is_pressed(&ButtonT::DownButton) {
			set_power(&MotorPortT::EV3PortB, -50);
		} else {
			stop(&MotorPortT::EV3PortA, false);
			stop(&MotorPortT::EV3PortB, false);
		}
	}
}
