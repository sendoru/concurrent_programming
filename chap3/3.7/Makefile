targets = 3_7_1_rwlock_spin.o 3_7_2_rwlock_pthreads \
	3_7_3_performance_RWLOCK \
	3_7_3_performance_RWLOCK_WR \
	3_7_3_performance_MUTEX \
	3_7_3_performance_EMPTY
CFLAGS = -O3
LDFLAGS = -pthread

# 다음 값을 변경하고 재컴파일 하면 소스 코드의 값도 변경된다
NUM_THREAD = 4
HOLDTIME = 100
DEFS = -DNUM_THREAD=$(NUM_THREAD) -DHOLDTIME=$(HOLDTIME)

all: $(targets)

3_7_3_performance_RWLOCK: 3_7_3_performance.c rwlock.c
	$(CC) $(CFLAGS) $(LDFLAGS) -DRWLOCK $(DEFS) $< -o $@

3_7_3_performance_RWLOCK_WR: 3_7_3_performance.c rwlock_wr.c
	$(CC) $(CFLAGS) $(LDFLAGS) -DRWLOCK_WR $(DEFS) $< -o $@

3_7_3_performance_MUTEX: 3_7_3_performance.c mutex.c
	$(CC) $(CFLAGS) $(LDFLAGS) -DMUTEX $(DEFS) $< -o $@

3_7_3_performance_EMPTY: 3_7_3_performance.c empty.c
	$(CC) $(CFLAGS) $(LDFLAGS) -DEMPTY $(DEFS) $< -o $@

.PHONY: clean
clean:
	rm -f $(targets)
