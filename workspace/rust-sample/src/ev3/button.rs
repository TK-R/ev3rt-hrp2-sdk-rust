pub enum ButtonT {
	LeftButton,
	RightButton,
	UpButton,
	DownButton,
	EnterButton,
	BackButton,
	TnumButton,
}

/// バッテリの電流を取得する
/// 戻り値：バッテリの電流（mA）
pub fn lap_button_is_pressed(b: ButtonT) -> bool {
	let b: u8 = match b {
		ButtonT::LeftButton => 0,
		ButtonT::RightButton => 1,
		ButtonT::UpButton => 2,
		ButtonT::DownButton => 3,
		ButtonT::EnterButton => 4,
		ButtonT::BackButton => 5,
		ButtonT::TnumButton => 6,
	};

	unsafe { ev3_button_is_pressed(b) }
}

extern "C" {
	fn ev3_button_is_pressed(button: u8) -> bool;
}
