
/// センサポートを表す番号
pub enum SensorPort {
	EV3Port1 = 0,
	EV3Port2,
	EV3Port3,
	EV3Port4,
	TNumSensorPort,
}

/// センサ種別を表す番号
pub enum SensorType {
	/// センサ未接続
	NoneSensor = 0,
	/// 超音波センサ
	UltraSonicSensor,
	/// ジャイロセンサ
	GyroSensor,
	/// タッチセンサ
	TouchSensor,
	/// カラーセンサ
	ColorSensor,
	/// センサタイプの数
	TNumSensorType, 
}

/// カラーセンサで識別できるカラーの番号
pub enum ColorId {
	/// 無色
	ColorNone,
	/// 黒
	ColorBlack,
	/// 青
	ColorBlue,
	/// 緑
	ColorGreen,
	/// 黄
	ColorYellow,
	/// 赤
	ColorRed,
	/// 白
	ColorWhite,
	/// 茶
	ColorBrown,
	/// 識別できるカラーの数
	TNumColor,
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct RGBRaw {
	pub red: u16,
	pub green:u16,
	pub blue:u16,
}

fn get_sensor_port(port: &SensorPort) -> u8 {
	match port {
		SensorPort::EV3Port1 => 0,
		SensorPort::EV3Port2 => 1,
		SensorPort::EV3Port3 => 2,
		SensorPort::EV3Port4 => 3,
		_ => 255,
	}
}

/// カラーセンサで環境光の強さを測定する．
pub fn color_sensor_get_ambient(port: SensorPort) -> u8{
	let port = get_sensor_port(&port);
	unsafe {
		ev3_color_sensor_get_ambient(port)
	}
}

/// カラーセンサでカラーを識別する． 
pub fn color_sensor_get_color(port: SensorPort) -> ColorId {
	let port = get_sensor_port(&port);
	let color = unsafe { ev3_color_sensor_get_color(port) };
	match color {
		1 => ColorId::ColorBlack,
		2 => ColorId::ColorBlue,
		3 => ColorId::ColorGreen,
		4 => ColorId::ColorYellow,
		5 => ColorId::ColorRed,
		6 => ColorId::ColorWhite,
		7 => ColorId::ColorBrown,
		_ => ColorId::ColorNone,
	}
}

/// カラーセンサで反射光の強さを測定する． 
pub fn color_sensor_get_reflect(port: &SensorPort) -> u8 {
	let port = get_sensor_port(&port);
	unsafe {ev3_color_sensor_get_reflect(port) }
}

/// カラーセンサでRGB Raw値を測定する． 
pub fn color_sensor_get_rgb_raw(port: &SensorPort, rgb_raw: &mut RGBRaw) {
	let port = get_sensor_port(&port);
	unsafe {ev3_color_sensor_get_rgb_raw(port, &mut *rgb_raw); }
}

/// ジャイロセンサで角位置を測定する． 
pub fn gyro_sensor_get_angle(port: &SensorPort) -> i16 {
	let port = get_sensor_port(&port);
	unsafe {ev3_gyro_sensor_get_angle(port) }
}

/// ジャイロセンサで各速度を測定する
pub fn gyro_sensor_get_rate(port: &SensorPort) -> i16 {
	let port = get_sensor_port(&port);
	unsafe {ev3_gyro_sensor_get_rate(port) }
}

/// ジャイロセンサの各位置をゼロにリセットする
pub fn gyro_sensor_reset(port :&SensorPort) -> i32 {
	let port = get_sensor_port(&port);
	unsafe {ev3_gyro_sensor_reset(port)}
}

/// タッチセンサの状態を検出する
pub fn touch_sensor_is_pressed(port: &SensorPort) -> bool {
	let port = get_sensor_port(&port);
	unsafe {ev3_touch_sensor_is_pressed(port)}
}

/// センサポートを設定する
pub fn config(port: &SensorPort, sensor_type:SensorType) -> i32 {
	let port = get_sensor_port(&port);
	unsafe {ev3_sensor_config(port, sensor_type as u8)}
}

/// センサポートのセンサタイプを取得する
pub fn sensor_get_type(port :&SensorPort) -> SensorType {
	let port = get_sensor_port(&port);
	let sensor_type = unsafe {ev3_sensor_get_type(port )};
	match sensor_type {
		1 => SensorType::UltraSonicSensor,
		2 => SensorType::GyroSensor,
		3 => SensorType::TouchSensor,
		4 => SensorType::ColorSensor,
		_ => SensorType::NoneSensor,
	}
}

/// 超音波センサで距離を測定する
pub fn ultrasonic_sensor_get_distance(port: &SensorPort) -> i16 {
	let port = get_sensor_port(&port);
	unsafe {ev3_ultrasonic_sensor_get_distance(port)}
}

/// 超音波センサで超音波信号を測定する
pub fn ultrasonic_sensor_listen(port: &SensorPort) -> bool {
	let port = get_sensor_port(&port);
	unsafe {ev3_ultrasonic_sensor_listen(port)}
}

extern "C" {
	fn ev3_color_sensor_get_ambient(port: u8) -> u8;
	fn ev3_color_sensor_get_color(port:u8) -> u8;
	fn ev3_color_sensor_get_reflect(port:u8) -> u8;
	fn ev3_color_sensor_get_rgb_raw(port:u8, raw: *mut RGBRaw) -> i32;

	fn ev3_gyro_sensor_get_angle(port:u8) -> i16;
	fn ev3_gyro_sensor_get_rate(port:u8) -> i16;
	fn ev3_gyro_sensor_reset(port:u8) -> i32;

	fn ev3_touch_sensor_is_pressed(port:u8) -> bool;
	fn ev3_ultrasonic_sensor_get_distance(port:u8) -> i16;
	fn ev3_ultrasonic_sensor_listen	(port:u8) -> bool;

	fn ev3_sensor_config(port:u8, sensor_type:u8) -> i32;
	fn ev3_sensor_get_type(port:u8) -> u32;

}