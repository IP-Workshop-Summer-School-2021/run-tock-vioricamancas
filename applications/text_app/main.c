/* vim: set sw=2 expandtab tw=80: */

#include <stdio.h>
#include "dots_test.h"

int main(void) {
  const char *a = "12";
  if (display_text_digit(a)) {
    printf("perf\r\n");
  } else  {
    printf("oh schmidt\r\n");
  }
  return 0;
}
