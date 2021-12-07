/* vim: set sw=2 expandtab tw=80: */

#include <stdio.h>
#include <stdlib.h>
#include <button.h>

static void button_callback(int btn_num,
                            int val,
                            __attribute__ ((unused)) int arg2,
                            __attribute__ ((unused)) void *ud) {
  int *p = 0x21000000;
  if (val == 1) {
    printf("shoot this ship down!\n");
    int a = *p;
    printf("Nein %d!\n",a);
  }
}

int main(void) {
  char stack[30];
  char *heap;
  int err;

  heap = malloc(30*sizeof(char));
  printf("stack addr %p heap addr %p\n", stack, &heap);

  err = button_subscribe(button_callback, NULL);
  if (err < 0) return err;

  // enable interrupts for button A
  button_enable_interrupt(0);

  return 0;
}
