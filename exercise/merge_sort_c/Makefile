cas = merge_sort.s merge_sort.o merge_sort
all: $(cas)
%.s: %.c
	$(CC) -O3 -S $<
LDFLAGS = -pthread -O3