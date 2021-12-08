#include <pthread.h> // ❶
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#define NUM_THREADS 10 // 생성할 스레드 수

// 스레드용 함수
void *thread_func(void *arg) { // ❷
    int id = (int)arg; // ❸
    for (int i = 0; i < 5; i++) { // ❹
        printf("id = %d, i = %d\n", id, i);
        sleep(1);
    }

    return "finished!"; // 반환값
}

int main(int argc, char *argv[]) {
    pthread_t v[NUM_THREADS]; // ⑤
    // 스레드 생성 ⑥
    for (int i = 0; i < NUM_THREADS; i++) {
        if (pthread_create(&v[i], NULL, thread_func, (void *)i) != 0) {
            perror("pthread_create");
            return -1;
        }
    }

    // 스레드 종료 대기 ⑦
    for (int i = 0; i < NUM_THREADS; i++) {
        char *ptr;
        if (pthread_join(v[i], (void **)&ptr) == 0) {
            printf("msg = %s\n", ptr);
        } else {
            perror("pthread_join");
            return -1;
        }
    }

    return 0;
}