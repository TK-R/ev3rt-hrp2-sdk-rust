pub mod battery;
pub mod button;
pub mod led;

pub fn lap_dly_tsk(msec: u32) -> i32 {
	unsafe { ev3_dly_tsk(msec) }
}

pub fn lap_speaker_play_tone(frequency: u16, duration: i32) -> i32 {
	unsafe { ev3_speaker_play_tone(frequency, duration) }
}

extern "C" {
	fn ev3_dly_tsk(msec: u32) -> i32;
	fn ev3_speaker_play_tone(frequency: u16, duration: i32) -> i32;
}
