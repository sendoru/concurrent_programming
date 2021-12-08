#include <pthread.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;
sigset_t set;

void *handler(void *arg) { // ❶
    pthread_detach(pthread_self()); // 디태치 ❷

    int sig;
    for (;;) {
        if (sigwait(&set, &sig) != 0) { // ❸
            perror("sigwait");
            exit(1);
        }
        printf("received signal: %d\n", sig);
        pthread_mutex_lock(&mutex);
        // 무언가의 처리
        pthread_mutex_unlock(&mutex);
    }

    return NULL;
}

void *worker(void *arg) { // ❹
    for (int i = 0; i < 10; i++) {
        pthread_mutex_lock(&mutex);
        // 무언가의 처리
        sleep(1);
        pthread_mutex_unlock(&mutex);
        sleep(1);
    }
    return NULL;
}

int main(int argc, char *argv[]) {
    // 프로세스 ID를 표시
    pid_t pid = getpid();
    printf("pid: %d\n", pid);

    // SIGUSR1 시그널을 블록으로 설정
    // 이 설정은 뒤에서 작성될 스레드에서도 이어진다 ❺
    sigemptyset(&set);
    sigaddset(&set, SIGUSR1);
    if (pthread_sigmask(SIG_BLOCK, &set, NULL) != 0) {
        perror("pthread_sigmask");
        return 1;
    }

    pthread_t th, wth;
    pthread_create(&th, NULL, handler, NULL);
    pthread_create(&wth, NULL, worker, NULL);
    pthread_join(wth, NULL);

    return 0;
}