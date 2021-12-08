# 3.4 세마포

## 파일

- `3_4_semaphore.c`: p## + 세마포 테스트 코드(책에는 실려있지 않음)
- `3_4_semaphore_llsc.c`: LL/SC 버전 세마포의 테스트 코드(책에는 실려있지 않음)
- `3_4_1_semaphore_llsc.S`: p##
- `3_4_2_posix_semaphore.c`: p##

## 컴파일

`make`를 실행하면 `.o` 파일, 및 실행 파일이 생성됩니다. LL/SC 버전은 AArch64 환경에서만 컴파일하기 바랍니다.

```sh
$ make
$ ls *.o
3_4_1_semaphore_llsc.o  3_4_semaphore.o
$ ls 3_4_semaphore 3_4_semaphore_llsc 3_4_2_posix_semaphore
3_4_2_posix_semaphore* 3_4_semaphore*         3_4_semaphore_llsc*
```

각 실행 파일을 실행하면 테스트가 수행됩니다.
