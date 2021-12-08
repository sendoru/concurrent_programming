#include "../3.2/3_2_2_tas.c"

void spinlock_acquire(volatile bool *lock) { // ❶
    for (;;) {
        while(*lock); // ❷
        if (!test_and_set(lock))
            break;
    }
}

void spinlock_release(bool *lock) {
    tas_release(lock);
}