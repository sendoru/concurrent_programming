#include <inttypes.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

// do_lock 함수의 내용 전환 ❶

#ifdef RWLOCK
    #include "rwlock.c"
#elif defined(RWLOCK_WR)
    #include "rwlock_wr.c"
#elif defined(MUTEX)
    #include "mutex.c"
#elif defined(EMPTY)
    #include "empty.c"
#endif

#include "barrier.c"

volatile int flag = 0; // 이 플래그가 0인 동안 루프

// 배리어 동기용 변수
volatile int waiting_1 = 0;
volatile int waiting_2 = 0;

uint64_t count[NUM_THREAD - 1]; // ❷

void *worker(void *arg) { // 워커 스레드용 함수 ❸
    uint64_t id = (uint64_t)arg;
    barrier(&waiting_1, NUM_THREAD); // 배리어 동기

    uint64_t n = 0; // ❹
    while (flag == 0) {
        do_lock(); // 필요하다면 록을 획득하고 대기❺
        n++;
    }
    count[id] = n; // 루프 횟수 기억

    barrier(&waiting_2, NUM_THREAD); // 배리어 동기

    return NULL;
}

void *timer(void *arg) { // 터이머스레드용 함수 ❻
    barrier(&waiting_1, NUM_THREAD); // 배리어 동기

    sleep(180);
    flag = 1;

    barrier(&waiting_2, NUM_THREAD); // 배리어 동기
    for (int i = 0; i < NUM_THREAD - 1; i++) {
        printf("%lu\n", count[i]);
    }

    return NULL;
}

int main() {
    // 어ㅓ커 스레드 실행
    for (uint64_t i = 0; i < NUM_THREAD - 1; i++) {
        pthread_t th;
        pthread_create(&th, NULL, worker, (void *)i);
        pthread_detach(th);
    }

    // 타이머 스레드 실행
    pthread_t th;
    pthread_create(&th, NULL, timer, NULL);
    pthread_join(th, NULL);

    return 0;
}