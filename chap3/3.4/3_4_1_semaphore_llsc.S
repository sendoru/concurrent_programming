// AArch64인 Mac에서도 동작하도록 일부를 수정했습니다

    .text
#ifdef __APPLE__
    .global _semaphore_acquire_llsc
#else
    .global semaphore_acquire_llsc
#endif

    .align 4

#ifdef __APPLE__
_semaphore_acquire_llsc:
#else
semaphore_acquire_llsc:
#endif

.LBB0_1:
    ldr     w8, [x0]     // while (*x0 > 3); ❶
    cmp     w8, #3
    b.hi    .LBB0_1
.Ltmp1:
    ldaxr   w2, [x0]     // w2 = [x0] ❷
    cmp     w2, #4
    b.lo    .Ltmp2       // if (w2 < 4) then goto .Ltmp2 ❸
    clrex                // clear exclusive
    b       .LBB0_1      // goto .LBB0_1
.Ltmp2:
    add     w2, w2, #1   // w2 = w2 + 1 ❹
    stlxr   w3, w2, [x0] // [x0] = w2
    cbnz    w3, .Ltmp1   // if (w3 != 0) then goto .Ltmp1
    ret