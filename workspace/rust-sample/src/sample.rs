use ev3::balancer::*;
use ev3::battery::*;
use ev3::button::*;
use ev3::ev3rt::*;
use ev3::lcd::*;
use ev3::led::*;
use ev3::motor::*;
use ev3::sensor::*;

/// 倒立振子機能を実施するサンプル
/// 無限ループで倒立し続ける
pub fn balancer_sample(
	l_motor: &MotorPort,
	r_motor: &MotorPort,
	touch: &SensorPort,
	gyro: &SensorPort,
) {
	balancer_init();

	// モータポートの初期化と、エンコーダのリセット
	motor_config(l_motor, &MotorTypeT::LargeMotor);
	motor_config(r_motor, &MotorTypeT::LargeMotor);
	reset_counts(l_motor);
	reset_counts(r_motor);

	// ジャイロセンサの初期化と、オフセット値のリセット
	sensor_config(gyro, &SensorType::GyroSensor);
	gyro_sensor_reset(gyro);

	// タッチセンサの初期化
	sensor_config(touch, &SensorType::TouchSensor);
	let mut r_pwm: i8 = 0;
	let mut l_pwm: i8 = 0;

	loop {
		while touch_sensor_is_pressed(touch) {
			gyro_sensor_reset(gyro);
			reset_counts(l_motor);
			reset_counts(r_motor);

			stop(l_motor, true);
			stop(r_motor, true);

			balancer_init();
		}

		lap_dly_tsk(4);

		let forward: f32 = 50.0;
		let turn: f32 = 0.0;
		let gyro_rate: f32 = gyro_sensor_get_rate(gyro) as f32;
		let gyro_offset: f32 = 0.0;
		let theta_m_l = get_counts(l_motor);
		let theta_m_r = get_counts(r_motor);
		let voltage: f32 = lap_battery_voltage_mv() as f32;

		balancer_control(
			forward,
			turn,
			gyro_rate,
			gyro_offset,
			theta_m_l,
			theta_m_r,
			voltage,
			&mut l_pwm,
			&mut r_pwm,
		);

		/*
		count = count + 1;
		if count > 100 {
			count = 0;
			lcd_clear(LCDColorT::EV3LCDWhite);
			draw_value("L:\0", l_pwm as i32, "-\0", 0, 0);
			draw_value("R:\0", r_pwm as i32, "-\0", 0, 15);
			draw_value("L:\0", theta_m_l as i32, "-\0", 0, 30);
			draw_value("E:\0", theta_m_r as i32, "-\0", 0, 40);
		}
		*/

		if l_pwm != 0 {
			set_power(l_motor, l_pwm as i32);
		} else {
			stop(l_motor, true);
		}
		if r_pwm != 0 {
			set_power(r_motor, r_pwm as i32);
		} else {
			stop(r_motor, true);
		}
	}
}

/// ジャイロセンサで取得した角速度と角度を表示するサンプル
/// 無限ループでLCDにジャイロセンサで取得した角速度と角度を表示し続ける
pub fn gyro_sample(gyro: SensorPort) {
	sensor_config(&gyro, &SensorType::GyroSensor);
	gyro_sensor_reset(&gyro);

	loop {
		lap_dly_tsk(100);
		lcd_clear(LCDColorT::EV3LCDWhite);

		let rate = gyro_sensor_get_rate(&gyro);
		let angle = gyro_sensor_get_angle(&gyro);

		draw_value("Rate\0", rate as i32, "rad/s\0", 0, 0);
		draw_value("Angle\0", angle as i32, "deg\0", 0, 15);
	}
}

/// カラーセンサの反射光の強さをLCDに出力するサンプル
/// 無限ループで反射光の強さをLCDに表示し続ける
pub fn color_sensor_reflect_sample(color_sensor_port: &SensorPort) {
	sensor_config(&color_sensor_port, &SensorType::ColorSensor);

	loop {
		let reflect = color_sensor_get_reflect(&color_sensor_port);
		lcd_clear(LCDColorT::EV3LCDWhite);
		draw_value("Reflect\0", reflect as i32, "%\0", 0, 0);
		lap_dly_tsk(100);
	}
}

/// カラーセンサのRGB生値のをLCDに出力するサンプル
/// 無限ループでRGBのRAW値をLCDに表示し続ける
pub fn color_sensor_raw_sample(color_sensor_port: SensorPort) {
	sensor_config(&color_sensor_port, &SensorType::ColorSensor);
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

/// タッチセンサの押下状態をLCDに出力するサンプル
/// 無限ループでタッチセンサの押下状態をLCDに表示し続ける
#[allow(dead_code)]
pub fn touch_sensor_sample(touch_sensor_port: SensorPort) {
	sensor_config(&touch_sensor_port, &SensorType::TouchSensor);

	loop {
		let pressed = touch_sensor_is_pressed(&touch_sensor_port);
		lcd_clear(LCDColorT::EV3LCDWhite);
		draw_value("Touch\0", pressed as i32, "-\0", 0, 0);
		lap_dly_tsk(100);
	}
}

/// 本体ボタンの押下状態とLEDの動作をサンプル
/// 無限ループで本体ボタンの押下状態に応じてLEDの点灯状態を制御する
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

/// バッテリ電圧と電流の状態を表示するサンプル
/// 無限ループでバッテリ電圧と電流をLCDに表示し続ける
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

/// 本体ボタンの押下状態に応じてモータを回転させるサンプル
#[allow(dead_code)]
pub fn button_motor_sample() {
	motor_config(&MotorPort::EV3PortA, &MotorTypeT::LargeMotor);
	motor_config(&MotorPort::EV3PortB, &MotorTypeT::MediumMotor);

	loop {
		lap_dly_tsk(100);

		if is_pressed(&ButtonT::RightButton) {
			set_power(&MotorPort::EV3PortA, 50);
		} else if is_pressed(&ButtonT::LeftButton) {
			set_power(&MotorPort::EV3PortA, -50);
		} else if is_pressed(&ButtonT::UpButton) {
			set_power(&MotorPort::EV3PortB, 50);
		} else if is_pressed(&ButtonT::DownButton) {
			set_power(&MotorPort::EV3PortB, -50);
		} else {
			stop(&MotorPort::EV3PortA, false);
			stop(&MotorPort::EV3PortB, false);
		}
	}
}
