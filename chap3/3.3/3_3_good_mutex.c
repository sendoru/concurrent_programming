#include "../3.2/3_2_2_tas.c"

bool lock = false; // 공유 변수

void some_func() {
retry:
    if (!test_and_set(&lock)) { // 검사 및 록 획득
        // 크리티컬 섹션
    } else {
        goto retry;
    }
    tas_release(&lock); // 록 반환
}