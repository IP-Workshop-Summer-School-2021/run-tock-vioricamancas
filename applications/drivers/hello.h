#pragma once

#include "tock.h"

bool hello_is_present(void);

void print_hello(void);

void add_ctr(int to_add);

void dec_ctr(int to_remove);

bool get_ctr(unsigned int *n);
