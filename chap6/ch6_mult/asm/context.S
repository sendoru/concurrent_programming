#ifdef __APPLE__ // Mac의 경우에는 함수명 처음에 언더스코어가 필요
    #define SET_CONTEXT _set_context
    #define SWITCH_CONTEXT _switch_context
#else
    #define SET_CONTEXT set_context
    #define SWITCH_CONTEXT switch_context
#endif

.global SET_CONTEXT // ❶
.global SWITCH_CONTEXT

SET_CONTEXT: // ❷
    // callee 저장 레지스터를 저장
    stp  d8,  d9, [x0] // ❸
    stp d10, d11, [x0, #16] // ❹
    stp d12, d13, [x0, #16 * 2]
    stp d14, d15, [x0, #16 * 3]
    stp x19, x20, [x0, #16 * 4]
    stp x21, x22, [x0, #16 * 5]
    stp x23, x24, [x0, #16 * 6]
    stp x25, x26, [x0, #16 * 7]
    stp x27, x28, [x0, #16 * 8]

    // 스택 포인터와 링크 레지스터를 저장
    mov x1, sp
    stp x30, x1, [x0, #16 * 9]

    // return 0 ❺
    mov x0, 0
    ret

SWITCH_CONTEXT: // ❻
    // callee 저장 레지스터를 복원
    ldp  d8,  d9, [x0] // ❼
    ldp d10, d11, [x0, #16] // ❽
    ldp d12, d13, [x0, #16 * 2]
    ldp d14, d15, [x0, #16 * 3]
    ldp x19, x20, [x0, #16 * 4]
    ldp x21, x22, [x0, #16 * 5]
    ldp x23, x24, [x0, #16 * 6]
    ldp x25, x26, [x0, #16 * 7]
    ldp x27, x28, [x0, #16 * 8]

    // 스택 포인터와 링크 레지스터를 복원
    ldp x30, x2, [x0, #16 * 9]
    mov sp, x2

    // return 1 ❾
    mov x0, 1
    ret