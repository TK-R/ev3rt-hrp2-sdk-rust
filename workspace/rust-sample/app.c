/**
 * This sample program balances a two-wheeled Segway type robot such as Gyroboy in EV3 core set.
 *
 * References:
 * http://www.hitechnic.com/blog/gyro-sensor/htway/
 * http://www.cs.bgu.ac.il/~ami/teaching/Lejos-2013/classes/src/lejos/robotics/navigation/Segoway.java
 */

#include "ev3api.h"
#include "app.h"

static int count = 0;
void test_ev3_cychdr(intptr_t idx)
{
    ev3_lcd_fill_rect(0, 0, EV3_LCD_WIDTH, EV3_LCD_HEIGHT, EV3_LCD_WHITE);
    char buf[100];
    sprintf(buf, "EV3CYC %d count", count);
    ev3_lcd_draw_string(buf, 0, 12);
}

void main_task(intptr_t unused)
{
    int a = 0;
    while (1)
    {
        dly_tsk(500);
        syslog(LOG_NOTICE, "a = %d, b = %d", a, twice(a));
        a++;
    }
}
