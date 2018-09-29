/// 倒立振子ライブラリを初期化する
pub fn balancer_init() {
	unsafe {
		balance_init();
	}
}

/// 倒立振子ライブラリから左右の出力PWM値を取得する
/// デカタイヤ対策のバックラッシュ除去処理を実行する
pub fn balancer_control(
	args_cmd_forward: f32,
	args_cmd_turn: f32,
	args_gyro: f32,
	args_gyro_offset: f32,
	args_theta_m_l: i32,
	args_theta_m_r: i32,
	args_battery: f32,
	ret_pwm_l: &mut i8,
	ret_pwm_r: &mut i8,
) {
	let backlash = 4; // バックラッシュの半分[deg]
	let mut lenc = args_theta_m_l;
	let mut renc = args_theta_m_r;

	if *ret_pwm_l < 0 {
		lenc += backlash;
	} else if *ret_pwm_l > 0 {
		lenc -= backlash;
	}
	if *ret_pwm_r < 0 {
		renc += backlash;
	} else if *ret_pwm_r > 0 {
		renc -= backlash;
	}

	unsafe {
		balance_control(
			args_cmd_forward,
			args_cmd_turn,
			args_gyro,
			args_gyro_offset,
			lenc as f32,
			renc as f32,
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
