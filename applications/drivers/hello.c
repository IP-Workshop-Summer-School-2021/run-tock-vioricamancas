#include "hello.h"
#include <stdio.h>
#include <button.h>

static const unsigned int HELLO_DRIVER_NR = 0xbabe;

static const char INC_CTR_CMD = 2;
static const char DEC_CTR_CMD = 3;

bool hello_is_present(void) {
    syscall_return_t res = command(HELLO_DRIVER_NR, 0,0,0);
    
    return res.type == TOCK_SYSCALL_SUCCESS;
}

void print_hello(void) {
    syscall_return_t res = command(HELLO_DRIVER_NR, 1, 0, 0);

    if (res.type != TOCK_SYSCALL_SUCCESS) {
        printf("Nasty\r\n");
    }
}


void add_ctr(int to_add){
    if (to_add == 1) {
        command(HELLO_DRIVER_NR, INC_CTR_CMD, 4, 0);
    } else {
         printf("wip\r\n");
    }
}

void dec_ctr(int to_remove) {
    if (to_remove == 1) {
        command(HELLO_DRIVER_NR, DEC_CTR_CMD, 1, 0);
    } else {
         printf("wip\r\n");
    }
}

bool get_ctr(unsigned int *n) {
    syscall_return_t res = command(HELLO_DRIVER_NR, 5, 0, 0);

    if (res.type == TOCK_SYSCALL_SUCCESS_U32) {
        *n = res.data[0];
        return true;
    }

    printf("error %d\r\n", res.type);
    return false;
}
