/* vim: set sw=2 expandtab tw=80: */

#include <stdio.h>
#include "dots_text.h"
#include "hello.h"

int main(void) {
  uint ctr;
  example_driver_action();
  if (hello_is_present()) {
  printf ("Hello's present yay!\r\n");
  } else {
  printf ("you hello-les bitch!\r\n");
  }
  print_hello();

  printf("Let's count!\r\n");

  add_ctr(1);
  add_ctr(1);

  if (get_ctr(&ctr)) {
    printf("Counter is %d\r\n", ctr);
  }
  dec_ctr(1);
  dec_ctr(1);
  printf("All good! Let's make it crack!");

  dec_ctr(1);

  return 0;
}
