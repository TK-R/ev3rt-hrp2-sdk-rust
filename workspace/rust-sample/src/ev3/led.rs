pub enum LEDColorT {
	LEFOff,
	LEDRed,
	LEDGreen,
	LEDOrange,
}

/// LEDライトのカラーを設定する
/// 戻り値：設定結果
pub fn set_led_color(led: &LEDColorT) -> i32 {
	let led = match led {
		LEDColorT::LEFOff => 0,
		LEDColorT::LEDRed => 1 << 0,
		LEDColorT::LEDGreen => 1 << 1,
		LEDColorT::LEDOrange => 1 << 0 | 1 << 1,
	};

	unsafe { ev3_led_set_color(led) }
}

extern "C" {
	fn ev3_led_set_color(color: u8) -> i32;
}
