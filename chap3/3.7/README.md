# 3.7 Readers-Writer록

## 파일

- `3_7_1_rwlock_spin.c`: p##, p##
- `3_7_2_rwlock_pthreads.c`: p##
- `3_7_3_performance.c`: p##
  - `empty.c`: p##
  - `mutex.c`: p##의 1번째
  - `rwlock.c`: p##의 2번째
  - `rwlock_wr.c`: p##의 3번째

## 컴파일

`make`를 실행하면 `.o` 파일, 또는 실행 파일이 생성됩니다.

```sh
$ make
$ ls *.o 3_7_2_rwlock_pthreads 3_7_3_performance_RWLOCK 3_7_3_performance_RWLOCK_WR 3_7_3_performance_MUTEX 3_7_3_performance_EMPTY
3_7_1_rwlock_spin.o          3_7_2_rwlock_pthreads*       3_7_3_performance_EMPTY*     3_7_3_performance_MUTEX*     3_7_3_performance_RWLOCK*    3_7_3_performance_RWLOCK_WR*
```
