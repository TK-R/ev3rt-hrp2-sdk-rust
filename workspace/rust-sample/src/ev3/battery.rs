/// バッテリの電流を取得する
/// 戻り値：バッテリの電流（mA）
pub fn lap_battery_current_ma() -> i32 {
	unsafe { ev3_battery_current_mA() }
}

/// バッテリの電圧を取得する
/// 戻り値：バッテリの電圧（mV）
pub fn lap_battery_voltage_mv() -> i32 {
	unsafe { ev3_battery_voltage_mV() }
}

extern "C" {
	fn ev3_battery_current_mA() -> i32;
	fn ev3_battery_voltage_mV() -> i32;
}
