#include <stdbool.h>

bool lock = false; // 공유 변수 ❶

void some_func() {
retry:
    if (!lock) { // ❷
        lock = true; // 록 획득
        // 크리티컬 섹션
    } else {
        goto retry;
    }
    lock = false; // 록 반환 ❸
}