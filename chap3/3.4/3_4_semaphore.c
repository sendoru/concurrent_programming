#define NUM 4

void semaphore_acquire(volatile int *cnt) { // ❶
    for (;;) {
        while (*cnt >= NUM); // ❷
        __sync_fetch_and_add(cnt, 1); // ❸
        if (*cnt <= NUM) // ❹
            break;
        __sync_fetch_and_sub(cnt, 1); // ❺
    }
}

void semaphore_release(int *cnt) {
    __sync_fetch_and_sub(cnt, 1); // ❻
}

#include "semtest.c"