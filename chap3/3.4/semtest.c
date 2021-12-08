// 테스트 코드

#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

#define NUM_THREADS 10 // 스레드 수
#define NUM_LOOP 10000 // 스레드 안의 루프 수

int cnt = 0; // 공유 변수

void *th(void *arg) {
    for (int i = 0; i < NUM_LOOP; i++) {
        semaphore_acquire(&cnt);
        if (cnt > 4) {
            printf("cnt = %d\n", cnt);
            exit(1);
        }
        semaphore_release(&cnt);
    }

    return NULL;
}

int main(int argc, char *argv[]) {
    // 스레드 생성
    pthread_t v[NUM_THREADS];
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_create(&v[i], NULL, th, NULL);
    }

    printf("OK!\n");

    return 0;
}