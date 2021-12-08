#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

pthread_rwlock_t rwlock = PTHREAD_RWLOCK_INITIALIZER; // ❶

void* reader(void *arg) { // Reader용 함수 ❷
    if (pthread_rwlock_rdlock(&rwlock) != 0) {
        perror("pthread_rwlock_rdlock"); exit(-1);
    }

    // 크리티컬 섹션(읽기만)

    if (pthread_rwlock_unlock(&rwlock) != 0) {
        perror("pthread_rwlock_unlock"); exit(-1);
    }

    return NULL;
}

void* writer(void *arg) { // Writer용 함수 ❸
    if (pthread_rwlock_wrlock(&rwlock) != 0) {
        perror("pthread_rwlock_wrlock"); exit(-1);
    }

    // 크리티컬 섹션(읽기)

    if (pthread_rwlock_unlock(&rwlock) != 0) {
        perror("pthread_rwlock_unlock"); exit(-1);
    }

    return NULL;
}

int main(int argc, char *argv[]) {
    // 스레드 생성
    pthread_t rd, wr;
    pthread_create(&rd, NULL, reader, NULL);
    pthread_create(&wr, NULL, writer, NULL);

    // 스레드 종료 대기
    pthread_join(rd, NULL);
    pthread_join(wr, NULL);

    // RW록 옵션 반환(해제)❹
    if (pthread_rwlock_destroy(&rwlock) != 0) {
        perror("pthread_rwlock_destroy"); return -1;
    }

    return 0;
}