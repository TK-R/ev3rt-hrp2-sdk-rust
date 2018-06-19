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

extern "C" {
	fn ev3_motor_config(port: u8, motor_type_t: u8) -> i32;
	fn ev3_motor_get_type(port: u8) -> u32;
	fn ev3_motor_get_counts(port: u8) -> i32;
	fn ev3_motor_reset_counts(port: u8) -> i32;
	fn ev3_motor_set_power(port: u8, i32 power) -> i32;
	fn ev3_motor_get_power (port: u8) -> i32;
	fn ev3_motor_stop(port: u8, break: bool);
	fn ev3_motor_rotate(port:u8, degrees: i32, speed_abs: u32, blocking: bool);
	fn ev3_motor_steer(left_motor: u8, right_motor: u8, power: i32, turn_ratio: i32);
}
