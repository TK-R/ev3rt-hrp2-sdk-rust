/// 倒立紳士ライブラリを初期化する
pub fn balancer_init() {
	unsafe {
		balance_init();
	}
}

pub fn balancer_control(
	args_cmd_forward: f32,
	args_cmd_turn: f32,
	args_gyro: f32,
	args_gyro_offset: f32,
	args_theta_m_l: f32,
	args_theta_m_r: f32,
	args_battery: f32,
	ret_pwm_l: &mut i8,
	ret_pwm_r: &mut i8,
) {
	unsafe {
		balance_control(
			args_cmd_forward,
			args_cmd_turn,
			args_gyro,
			args_gyro_offset,
			args_theta_m_l,
			args_theta_m_r,
			args_battery,
			&mut *ret_pwm_l,
			&mut *ret_pwm_r,
		);
	}
}

extern "C" {
	fn balance_init();
	fn balance_control(
		args_cmd_forward: f32,
		args_cmd_turn: f32,
		args_gyro: f32,
		args_gyro_offset: f32,
		args_theta_m_l: f32,
		args_theta_m_r: f32,
		args_battery: f32,
		ret_pwm_l: *mut i8,
		ret_pwm_r: *mut i8,
	);

}
