const LCD_WIDTH: i32 = 178;
const LCD_HEIGHT: i32 = 128;
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

// 値に名称と単位を付与してLCDに出力する
pub fn draw_value(name: &str, value:i32, unit:&str, x:i32, y:i32) -> i32
{
	let name = CString::new(name).unwrap();
	let unit = CString::new(unit).unwrap();
	unsafe {
		ev3_lcd_draw_int_value(name.into_raw(), value, unit.into_raw(), x, y)
	}
}

pub fn clear(color:LCDColorT) -> i32 {
	fill_rect(0, 0, LCD_WIDTH, LCD_HEIGHT, color)
}

pub fn fill_rect(x: i32, y: i32, w: i32, h: i32, color:LCDColorT) -> i32 {
	let color = match color {
		LCDColorT::EV3LCDWhite => 0,
		LCDColorT::EV3LCDBlack => 1,
	};
	
	unsafe {
		ev3_lcd_fill_rect(x, y, w, h, color)
	}

}


extern "C" {
	fn ev3_lcd_set_font(font: u8) -> i32;
	fn ev3_font_get_size(font: u8, p_width: &[u8], p_height: &[u8]) -> i32;
	fn ev3_image_load(p_memfile: &[u8], p_image: &[u8]) -> i32;
	fn ev3_image_free(p_image: &[u8]) -> i32;
	fn ev3_lcd_draw_line(x0: i32, y0: i32, x1: i32, y1: i32) -> i32;
	fn ev3_lcd_fill_rect(x: i32, y: i32, w: i32, h: i32, color: u8) -> i32;
	fn ev3_lcd_draw_image(p_image: &[u8], x: i32, y: i32) -> i32;

	// app.cに定義した関数
	fn ev3_lcd_draw_int_value(name: *const c_char, value:i32, unit:*const c_char, x: i32, y: i32) -> i32;
	fn ev3_lcd_draw_double_value(name: *const c_char, value:f64, unit:*const c_char, x:i32, y:i32 ) -> i32;
}
