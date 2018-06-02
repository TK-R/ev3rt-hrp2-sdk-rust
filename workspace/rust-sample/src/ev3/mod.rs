pub mod battery;
pub mod button;
pub mod led;

pub fn lap_dly_tsk(msec: u32) -> i32 {
	unsafe { ev3_dly_tsk(msec) }
}

pub fn lap_speaker_play_tone(frequency: u16, duration: i32) -> i32 {
	unsafe { ev3_speaker_play_tone(frequency, duration) }
}

pub fn lap_syslog(prio: u32, format: &str) {}

extern "C" {
	fn ev3_dly_tsk(msec: u32) -> i32;
	fn ev3_syslog(prio: u32, format: *mut u8);
	fn ev3_speaker_play_tone(frequency: u16, duration: i32) -> i32;
}
