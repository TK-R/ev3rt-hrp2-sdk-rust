pub enum MotorPortT {
	EV3PortA = 0,
	EV3PortB,
	EV3PortC,
	EV3PortD,
	TNumMotorPort,
}

pub enum MotorTypeT {
	NoneMotor = 0,
	MediumMotor,
	LargeMotor,
	UnregulatedMotor,
	TNumMotorType,
}

/// モータのポートを型変換する
fn get_motor_port(motor_port: MotorPortT) -> u8 {
	match motor_port {
		MotorPortT::EV3PortA => 0,
		MotorPortT::EV3PortB => 1,
		MotorPortT::EV3PortC => 2,
		MotorPortT::EV3PortD => 3,
		MotorPortT::TNumMotorPort => 4,
	}
}

/// モータのタイプを型変換する
fn get_motor_type(motor_type: MotorTypeT) -> u8 {
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
pub fn lap_motor_config(motor_port: MotorPortT, motor_type: MotorTypeT) -> i32 {
	let motor_port = get_motor_port(motor_port);
	let motor_type = get_motor_type(motor_type);

	unsafe { ev3_motor_config(motor_port, motor_type) }
}

extern "C" {
	fn ev3_motor_config(port: u8, motor_type_t: u8) -> i32;
	fn ev3_motor_get_type(port: u8) -> u32;
	fn ev3_motor_get_counts(port: u8) -> i32;
	fn ev3_motor_reset_counts(port: u8) -> i32;
	fn ev3_motor_set_power(port: u8, power: i32) -> i32;
	fn ev3_motor_get_power(port: u8) -> i32;
	fn ev3_motor_stop(port: u8, motor_brake: bool);
	fn ev3_motor_rotate(port: u8, degrees: i32, speed_abs: u32, blocking: bool);
	fn ev3_motor_steer(left_motor: u8, right_motor: u8, power: i32, turn_ratio: i32);
}
