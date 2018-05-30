#[link(name = "ev3api")]

extern "C" {
	#[allow(dead_code)]
	fn ev3_dly_tsk(msec: u32) -> i32;

	#[allow(dead_code)]
	fn ev3_speaker_play_tone(frequency: u16, duration: i32) -> i32;
}

pub fn lap_dly_tsk(msec: u32) -> i32 {
	unsafe { ev3_dly_tsk(msec) }
}

pub fn lap_speaker_play_tone(frequency: u16, duration: i32) -> i32 {
	unsafe { ev3_speaker_play_tone(frequency, duration) }
}
