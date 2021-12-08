# 3.3 뮤텍스

## 파일

- `3_3_bad_mutex.c`: p##
- `3_3_good_mutex.c`: p##
- `3_3_1_spinlock_1.c`: p##
- `3_3_1_spinlock_2.c`: p##
- `3_3_1_use_spinlock.c`: p##
- `3_3_2_pthreads_mutex.c`: p##

## 컴파일

`make`를 실행하면 `.o` 파일, 및 실행 파일이 생성됩니다.

```sh
$ make
$ ls *.o
3_3_1_spinlock_1.o    3_3_1_spinlock_2.o    3_3_1_use_spinlock.o  3_3_bad_mutex.o       3_3_good_mutex.o
$ ls 3_3_2_pthreads_mutex
3_3_2_pthreads_mutex*
```
