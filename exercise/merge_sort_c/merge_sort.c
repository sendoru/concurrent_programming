#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define MAX_DEPTH 4
#define INSERTION_SORT_THRESHOLD 32

// if segment is small enough, use insertion sort
void insertion_sort(int *arr, int left, int right)
{
    for (int i = left + 1; i <= right; i++) {
        int key = arr[i];
        int j = i - 1;
        while (j >= left && arr[j] > key) {
            arr[j + 1] = arr[j];
            j--;
        }
        arr[j + 1] = key;
    }
}

// function for multi-thread merge sort
void merge(int *arr, int left, int mid, int right)
{
    int n1 = mid - left + 1;
    int n2 = right - mid;

    int *L = (int *)malloc(sizeof(int) * n1);
    int *R = (int *)malloc(sizeof(int) * n2);

    for (int i = 0; i < n1; i++)
        L[i] = arr[left + i];
    for (int i = 0; i < n2; i++)
        R[i] = arr[mid + 1 + i];

    int i = 0, j = 0, k = left;
    while (i < n1 && j < n2) {
        if (L[i] <= R[j]) {
            arr[k] = L[i];
            i++;
        }
        else {
            arr[k] = R[j];
            j++;
        }
        k++;
    }

    while (i < n1) {
        arr[k] = L[i];
        i++;
        k++;
    }

    while (j < n2) {
        arr[k] = R[j];
        j++;
        k++;
    }

    free(L);
    free(R);
}

void merge_sort_single_thread(int *arr, int left, int right)
{
    if (right - left + 1 <= INSERTION_SORT_THRESHOLD) {
        insertion_sort(arr, left, right);
        return;
    }
    if (left < right) {
        int mid = left + (right - left) / 2;

        merge_sort_single_thread(arr, left, mid);
        merge_sort_single_thread(arr, mid + 1, right);

        merge(arr, left, mid, right);
    }
}

void *__merge_sort_multi_thread(void *arg)
{
    int *arg_ = (int *)arg;
    int *arr = *(int **)arg;
    int left = arg_[2];
    int right = arg_[3];
    int depth = arg_[4];

    if (right - left + 1 <= INSERTION_SORT_THRESHOLD) {
        insertion_sort(arr, left, right);
        return NULL;
    }

    if (left < right) {
        int mid = left + (right - left) / 2;

        if (depth < MAX_DEPTH) {
            int arg1[] = {0, 0, left, mid, depth + 1};
            int arg2[] = {0, 0, mid + 1, right, depth + 1};
            *(int **)arg1 = arr;
            *(int **)arg2 = arr;
            pthread_t th1, th2;
            if (pthread_create(&th1, NULL, __merge_sort_multi_thread,
                               (void *)arg1) != 0) {
                perror("pthread_create");
                exit(-1);
            }
            if (pthread_create(&th2, NULL, __merge_sort_multi_thread,
                               (void *)arg2) != 0) {
                perror("pthread_create");
                exit(-1);
            }

            // spin until both threads finish
            if (pthread_join(th1, NULL) != 0) {
                perror("pthread_join");
                exit(-1);
            }
            if (pthread_join(th2, NULL) != 0) {
                perror("pthread_join");
                exit(-1);
            }
        }
        else {
            merge_sort_single_thread(arr, left, mid);
            merge_sort_single_thread(arr, mid + 1, right);
        }

        merge(arr, left, mid, right);
    }

    return NULL;
}

// depth: current depth of the recursion
// create new thread only if depth < MAX_DEPTH
// warning: parent thread should wait for child threads to finish
void merge_sort_multi_thread(int *arr, int left, int right)
{
    int arg[5] = {0, 0, left, right, 0};
    *(int **)arg = arr;
    __merge_sort_multi_thread((void *)arg);
}

int main(int argc, char *argv[])
{
    int n;
    if (argc < 2) {
        printf("Usage: %s <n>\n", argv[0]);
        return -1;
    }
    n = atoi(argv[1]);
    int *arr = (int *)malloc(sizeof(int) * n);

    for (int i = 0; i < n; i++)
        arr[i] = rand() % (3 * n);

    // copy arr to arr2
    int *arr2 = (int *)malloc(sizeof(int) * n);
    for (int i = 0; i < n; i++)
        arr2[i] = arr[i];

    if (n <= 100) {
        printf("Generated array is \n");
        for (int i = 0; i < n; i++)
            printf("%d ", arr[i]);
        printf("\n");
    }
    else {
        printf("n = %d, skipped printing elements for too large n.\n", n);
    }

    struct timespec start, end;

    // check time for multi-thread merge sort
    // the clock() function returns SUM of the number of clock ticks of ALL
    // CORES elapsed since the program was launched so the time measured by
    // clock() function is not accurate when multi-threading is used instead,
    // use gettimeofday() function or clock_gettime() function
    clock_gettime(CLOCK_REALTIME, &start);
    int arg[5] = {0, 0, 0, n - 1, 0};
    *(int **)arg = arr;
    merge_sort_multi_thread(arr, 0, n - 1);
    clock_gettime(CLOCK_REALTIME, &end);
    printf("Time for multi-thread merge sort: %lf\n",
           (end.tv_sec - start.tv_sec) +
               (double)(end.tv_nsec - start.tv_nsec) / 1000000000.);

    // check time for single-thread merge sort
    clock_gettime(CLOCK_REALTIME, &start);
    merge_sort_single_thread(arr2, 0, n - 1);
    clock_gettime(CLOCK_REALTIME, &end);
    printf("Time for single-thread merge sort: %lf\n",
           (end.tv_sec - start.tv_sec) +
               (double)(end.tv_nsec - start.tv_nsec) / 1000000000.);

    if (n <= 100) {
        printf("Sorted array is \n");
        for (int i = 0; i < n; i++)
            printf("%d ", arr[i]);
        printf("\n");
    }
    // check if the array is sorted
    bool sorted = true;
    for (int i = 0; i < n - 1; i++) {
        if (arr[i] > arr[i + 1]) {
            printf("Array is not sorted\n");
            sorted = false;
            break;
        }
    }

    if (sorted) {
        printf("Array is sorted\n");
    }

    return 0;
}