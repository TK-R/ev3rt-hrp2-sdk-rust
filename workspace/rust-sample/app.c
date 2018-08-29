/**
 * This sample program balances a two-wheeled Segway type robot such as Gyroboy in EV3 core set.
 *
 * References:
 * http://www.hitechnic.com/blog/gyro-sensor/htway/
 * http://www.cs.bgu.ac.il/~ami/teaching/Lejos-2013/classes/src/lejos/robotics/navigation/Segoway.java
 */

#include "ev3api.h"

#include <string.h>
#include <syssvc/serial.h>

#include "app.h"

// dly_tskのラッパ関数
ER ev3_dly_tsk(uint32_t msec)
{
    return dly_tsk(msec);
}

// serial_wri_datのラッパ関数
ER ev3_serial_wri_dat(ID portid, const char *buf, uint_t len)
{
    return serial_wri_dat(portid, buf, len);
}

// syslogのラッパ関数
void ev3_syslog(uint_t prio, const char *format, ...)
{
    syslog(prio, format);
}

// Rustから文字列をLCDに出力するためのラッパ関数
int ev3_lcd_draw_int_value(const char *name, int value, const char *unit, int x, int y)
{
    char buff[256];
    sprintf(buff, "%s:%d [%s]", name, value, unit);
    return ev3_lcd_draw_string(buff, x, y);
}

// Rustから文字列をLCDに出力するためのラッパ関数
int ev3_lcd_draw_double_value(const char *name, double value, const char *unit, int x, int y)
{
    char buff[256];
    sprintf(buff, "%s:%lf [%s]", name, value, unit);
    return ev3_lcd_draw_string(buff, x, y);
}

// rcsringエラー時にコールするためのアボート関数
void abort()
{
}
