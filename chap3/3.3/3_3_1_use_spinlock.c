#include "3_3_1_spinlock_2.c"

bool lock = false; // 공유 변수

void some_func() {
    for (;;) {
        spinlock_acquire(&lock); // 록 획득 ❶
        // 크리티컬 섹션 ❷
        spinlock_release(&lock); // 록 반환 ❸
    }
}