#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct MyWrapper MyWrapper;

struct MyWrapper *my_wrapper_create(void);

void my_wrapper_free(struct MyWrapper *ptr);
