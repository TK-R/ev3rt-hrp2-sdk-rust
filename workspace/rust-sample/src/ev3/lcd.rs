const LCD_WIDTH: u8 = 178;
const LCD_HEIGHT: u8 = 128;

struct ImageT {
	width: i32,
	height: i32,
	data: *const u8,
}
