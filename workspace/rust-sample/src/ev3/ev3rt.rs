const EV3_SERIAL_BT: u8 = 2;

pub fn lap_dly_tsk(msec: u32) -> i32 {
	unsafe { ev3_dly_tsk(msec) }
}

pub fn lap_speaker_play_tone(frequency: u16, duration: i32) -> i32 {
	unsafe { ev3_speaker_play_tone(frequency, duration) }
}

pub fn lap_syslog(_string: &str) {
	let y = [1u8, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36];
	let y = &y as *const [u8; 7];

	unsafe {
		ev3_serial_wri_dat(EV3_SERIAL_BT, y, 6);
	}
}

pub fn lap_is_connect() -> bool {
	unsafe { ev3_bluetooth_is_connected() }
}

extern "C" {
	fn ev3_serial_wri_dat(id: u8, buf: *const [u8], size: usize) -> u32;
	fn ev3_bluetooth_is_connected() -> bool;
	fn ev3_dly_tsk(msec: u32) -> i32;
	fn ev3_syslog(prio: u32, format: *mut u8);
	fn ev3_speaker_play_tone(frequency: u16, duration: i32) -> i32;
}
