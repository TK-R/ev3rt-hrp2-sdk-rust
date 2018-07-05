const LCD_WIDTH: u8 = 178;
const LCD_HEIGHT: u8 = 128;
use rcstring::{c_char, CString};
struct ImageT {
	width: i32,
	height: i32,
	data: *const u8,
}

pub enum LCDFontT {
	EV3FontSmall,
	EV3FontLarge,
}

pub enum LCDColorT {
	EV3LCDWhite,
	EV3LCDBlack,
}

// デフォルトのフォントを設定する
pub fn set_font(font: LCDFontT) -> i32 {
	let font = match font {
		LCDFontT::EV3FontSmall => 0,
		LCDFontT::EV3FontLarge => 1,
	};

	unsafe { ev3_lcd_set_font(font) }
}

pub fn draw_string(s: &str, x: i32, y: i32) -> i32 {
	let ss = CString::new("Rust on EV3RT!!\0").unwrap();
	unsafe { ev3_lcd_draw_string(ss.into_raw(), x, y) }
}

extern "C" {
	fn ev3_lcd_set_font(font: u8) -> i32;
	fn ev3_font_get_size(font: u8, p_width: &[u8], p_height: &[u8]) -> i32;
	fn ev3_image_load(p_memfile: &[u8], p_image: &[u8]) -> i32;
	fn ev3_image_free(p_image: &[u8]) -> i32;
	fn ev3_lcd_draw_string(string: *const c_char, x: i32, y: i32) -> i32;
	fn ev3_lcd_draw_line(x0: i32, y0: i32, x1: i32, y1: i32) -> i32;
	fn ev3_lcd_fill_rect(x: i32, y: i32, w: i32, h: i32, color: u8) -> i32;
	fn ev3_lcd_draw_image(p_image: &[u8], x: i32, y: i32) -> i32;
}
