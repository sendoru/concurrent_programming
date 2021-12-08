// LL/SC를 이용한 세마포의 테스트 코드입니다.
// 이 코드는 책에는 실려있지 않습니다.

#define NUM 4

void semaphore_acquire_llsc(volatile int *cnt);

void semaphore_acquire(int *cnt) {
    semaphore_acquire_llsc(cnt);
}

void semaphore_release(int *cnt) {
    __sync_fetch_and_sub(cnt, 1);
}

#include "semtest.c"