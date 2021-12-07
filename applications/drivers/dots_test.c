#include "dots_test.h"

#define DOTS_TEXT_DRIVER_NUM 0xa0002

static void fn_done(int arg1, int a2,int a3, void *user_data) {
    printf("upcall\r\n");
}

bool display_text_digit(const char *digit) {
    digit;
    subscribe_return_t sr = subscribe(DOTS_TEXT_DRIVER_NUM, 0, fn_done, 0);
    return sr.success;
}