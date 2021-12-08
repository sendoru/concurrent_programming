#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

pthread_mutex_t mut = PTHREAD_MUTEX_INITIALIZER; // ❶

void* some_func(void *arg) { // 스레드용 함수
    if (pthread_mutex_lock(&mut) != 0) { // ❷
        perror("pthread_mutex_lock"); exit(-1);
    }

    // 크리티컬 섹션

    if (pthread_mutex_unlock(&mut) != 0) { // ❸
        perror("pthread_mutex_unlock"); exit(-1);
    }

    return NULL;
}

int main(int argc, char *argv[]) {
    // 스레드 생성
    pthread_t th1, th2;
    if (pthread_create(&th1, NULL, some_func, NULL) != 0) {
        perror("pthread_create"); return -1;
    }

    if (pthread_create(&th2, NULL, some_func, NULL) != 0) {
        perror("pthread_create"); return -1;
    }

    // 스레드 종료 대기
    if (pthread_join(th1, NULL) != 0) {
        perror("pthread_join"); return -1;
    }

    if (pthread_join(th2, NULL) != 0) {
        perror("pthread_join"); return -1;
    }

    // 뮤텍스 객체 반환
    if (pthread_mutex_destroy(&mut) != 0) { // ❹
        perror("pthread_mutex_destroy"); return -1;
    }

    return 0;
}