#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Wrapper for `MyStruct`.
 */
typedef struct MyWrapper MyWrapper;

/**
 * Create a new instance of the wrapper.
 */
struct MyWrapper *my_wrapper_create(void);

/**
 * Free the instance of the wrapper.
 */
void my_wrapper_free(struct MyWrapper *ptr);

void test_func(struct MyWrapper *spam);
