void do_lock() {
    for (uint64_t i = 0; i < HOLDTIME; i++) {
        asm volatile("nop"); // 아무것도 하지 않음
    }
}