#include "../../../chap3/3.3/3_3_1_spinlock_2.c"

#include <assert.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

// 재진입 가능한 뮤텍스용 타입 ❶
struct reent_lock {
    bool lock; // 록용 공유 변수
    int id;    // 현재 록을 획득 중인 스레드 ID, 0이 아니면 록 획득중임
    int cnt;   // 재귀 록 카운트
};

// 재귀 록 획득 함수
void reentlock_acquire(struct reent_lock *lock, int id) {
    // 록 획득 중이고 자신이 획득 중인지 판정 ❷
    if (lock->lock && lock->id == id) {
        // 자신이 획등 중이면 카운트를 인크리먼트
        lock->cnt++;
    } else {
        // 어떤 스레드도 혹을 획득하지 않았거나,
        // 다른 스레드가 록 획득 중이면 록 획득
        spinlock_acquire(&lock->lock);
        // 록윽 획득하면 자신의 스레드 ID를 설정하고
        // 마운트를 인크리먼트
        lock->id = id;
        lock->cnt++;
    }
}

// 재귀 록 해제 함수
void reentlock_release(struct reent_lock *lock) {
    // 카운트를 디크리먼트하고,
    // 해당 카운트가 0이 되면 록 해제 ❸
    lock->cnt--;
    if (lock->cnt == 0) {
        lock->id = 0;
        spinlock_release(&lock->lock);
    }
}

struct reent_lock lock_var; // 록용 공유 변수

// n회 재귀적으로 호출해 록을 거는 테스트 함수
void reent_lock_test(int id, int n) {
    if (n == 0)
        return;

    // 재귀 록
    reentlock_acquire(&lock_var, id);
    reent_lock_test(id, n - 1);
    reentlock_release(&lock_var);
}

// 스레드용 함수
void *thread_func(void *arg) {
    int id = (int)arg;
    assert(id != 0);
    for (int i = 0; i < 10000; i++) {
        reent_lock_test(id, 10);
    }
    return NULL;
}

int main(int argc, char *argv[]) {
    pthread_t v[NUM_THREADS];
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_create(&v[i], NULL, thread_func, (void *)(i + 1));
    }
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_join(v[i], NULL);
    }
    return 0;
}