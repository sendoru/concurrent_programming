#include <stdlib.h>

void barrier(volatile int *cnt, int max) { // ❶
    __sync_fetch_and_add(cnt, 1); // ❷
    while (*cnt < max); // ❸
}
