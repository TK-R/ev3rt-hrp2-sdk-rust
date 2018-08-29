/// モータポートを表す番号
pub enum MotorPortT {
	///　ポートA
	EV3PortA = 0,
	/// ポートB
	EV3PortB,
	/// ポートC
	EV3PortC,
	/// ポートD
	EV3PortD,
	/// モータポートの数
	TNumMotorPort,
}

/// モータタイプを表す番号
pub enum MotorTypeT {
	/// モータ未接続
	NoneMotor = 0,
	/// サーボモータM
	MediumMotor,
	/// サーボモータL
	LargeMotor,
	/// 未調整モータ
	UnregulatedMotor,
	/// モータタイプの数
	TNumMotorType,
}

/// モータのポートを型変換する
fn get_motor_port(motor_port: &MotorPortT) -> u8 {
	match motor_port {
		MotorPortT::EV3PortA => 0,
		MotorPortT::EV3PortB => 1,
		MotorPortT::EV3PortC => 2,
		MotorPortT::EV3PortD => 3,
		MotorPortT::TNumMotorPort => 4,
	}
}

/// モータのタイプを型変換する
fn get_motor_type(motor_type: &MotorTypeT) -> u8 {
	match motor_type {
		MotorTypeT::NoneMotor => 0,
		MotorTypeT::MediumMotor => 1,
		MotorTypeT::LargeMotor => 2,
		MotorTypeT::UnregulatedMotor => 3,
		MotorTypeT::TNumMotorType => 4,
	}
}

/// モータポートを設定する。
/// モータポートに接続しているモータのタイプを決定する。既に設定した場合も新しいモータタイプを指定できる。
pub fn config(motor_port: &MotorPortT, motor_type: &MotorTypeT) -> i32 {
	let motor_port = get_motor_port(&motor_port);
	let motor_type = get_motor_type(&motor_type);

	unsafe { ev3_motor_config(motor_port, motor_type) }
}

/// モータの角位置を取得する
/// 不正のモータポート番号をしていた場合、常に0を返す
pub fn get_counts(motor_port: &MotorPortT) -> i32 {
	let motor_port = get_motor_port(&motor_port);
	unsafe { ev3_motor_get_counts(motor_port) }
}

/// モータのパワーを取得する
/// 不正のモータポート番号を指定した場合、常に0を返す
pub fn get_power(motor_port: &MotorPortT) -> i32 {
	let motor_port = get_motor_port(&motor_port);
	unsafe { ev3_motor_get_power(motor_port) }
}

/// モータポートのタイプを取得する
pub fn get_type(motor_port: &MotorPortT) -> MotorTypeT {
	let motor_port = get_motor_port(&motor_port);
	unsafe {
		let motor_type = ev3_motor_get_type(motor_port);
		match motor_type {
			0 => MotorTypeT::NoneMotor,
			1 => MotorTypeT::MediumMotor,
			2 => MotorTypeT::LargeMotor,
			_ => MotorTypeT::UnregulatedMotor,
		}
	}
}

/// モータの角位置をゼロにリセットする
/// モータの角位置センサの値を設定するだけ、モータの実際のパワーと位置に影響を与えない
pub fn reset_counts(motor_port: &MotorPortT) -> i32 {
	let motor_port = get_motor_port(&motor_port);
	unsafe { ev3_motor_reset_counts(motor_port) }
}

/// モータを指定した角度だけ回転させる
pub fn rotate(motor_port: &MotorPortT, degrees: i32, speed_abs: u32, blocking: bool) -> i32 {
	let motor_port = get_motor_port(&motor_port);
	unsafe { ev3_motor_rotate(motor_port, degrees, speed_abs, blocking) }
}

/// モータのパワーを設定する
pub fn set_power(motor_port: &MotorPortT, power: i32) -> i32 {
	let motor_port = get_motor_port(&motor_port);
	unsafe { ev3_motor_set_power(motor_port, power) }
}

/// 2つのモータでロボットのステアリング操作を行う
pub fn steer(left_motor: &MotorPortT, right_motor: &MotorPortT, power: i32, turn_ratio: i32) -> i32 {
	let left_motor = get_motor_port(&left_motor);
	let right_motor = get_motor_port(&right_motor);
	unsafe { ev3_motor_steer(left_motor, right_motor, power, turn_ratio) }
}

/// モータを停止する
pub fn stop(motor_port: &MotorPortT, motor_brake: bool) -> i32 {
	let motor_port = get_motor_port(&motor_port);
	unsafe { ev3_motor_stop(motor_port, motor_brake) }
}

extern "C" {
	fn ev3_motor_config(port: u8, motor_type_t: u8) -> i32;
	fn ev3_motor_get_type(port: u8) -> u32;
	fn ev3_motor_get_counts(port: u8) -> i32;
	fn ev3_motor_reset_counts(port: u8) -> i32;
	fn ev3_motor_set_power(port: u8, power: i32) -> i32;
	fn ev3_motor_get_power(port: u8) -> i32;
	fn ev3_motor_stop(port: u8, motor_brake: bool) -> i32;
	fn ev3_motor_rotate(port: u8, degrees: i32, speed_abs: u32, blocking: bool) -> i32;
	fn ev3_motor_steer(left_motor: u8, right_motor: u8, power: i32, turn_ratio: i32) -> i32;
}
