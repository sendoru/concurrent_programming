use std::ptr::null_mut;

// 스택의 노드. 리스트 구조로 관리 ❶
#[repr(C)]
struct Node<T> {
    next: *mut Node<T>,
    data: T,
}

// 스택의 선두 ❷
#[repr(C)]
pub struct StackHead<T> {
    head: *mut Node<T>,
}

impl<T> StackHead<T> {
    fn new() -> Self {
        StackHead { head: null_mut() }
    }

    pub fn push(&mut self, v: T) { // ❸
        //추가할 노드를 작성
        let node = Box::new(Node {
            next: null_mut(),
            data: v,
        });

        // Box 타입의 값으로부터 포인터를 꺼낸다
        let ptr = Box::into_raw(node) as *mut u8 as usize;

        // 포인터의 포인터를 취득
        // head에 저장되어 있는 메모리를 LL/SC
        let head = &mut self.head as *mut *mut Node<T> as *mut u8 as usize;

        // LL/SC를 이용한 push ❹
        unsafe {
            asm!("1:
                  ldxr {next}, [{head}] // next = *head
                  str {next}, [{ptr}]   // *ptr = next
                  stlxr w10, {ptr}, [{head}] // *head = ptr
                  // if tmp != 0 then goto 1
                  cbnz w10, 1b",
                next = out(reg) _,
                ptr = in(reg) ptr,
                head = in(reg) head,
                out("w10") _)
        };
    }

    pub fn pop(&mut self) -> Option<T> { // ❺
        unsafe {
            // 포인터의 포인터를 취득
            // head에 저장된 메모리를 LL/SC
            let head = &mut self.head as *mut *mut Node<T> as *mut u8 as usize;

            // pop한 노드로의 주소를 저장
            let mut result: usize;

            // LL/SC을 이용한 pop ❻
            asm!("1:
                  ldaxr {result}, [{head}] // result = *head
                  // if result != NULL then goto 2
                  cbnz {result}, 2f

                  // if NULL
                  clrex // clear exclusive
                  b 3f  // goto 3

                  // if not NULL
                  2:
                  ldr {next}, [{result}]     // next = *result
                  stxr w10, {next}, [{head}] // *head = next
                  // if tmp != 0 then goto 1
                  cbnz w10, 1b

                  3:",
                next = out(reg) _,
                result = out(reg) result,
                head = in(reg) head,
                out("w10") _);

            if result == 0 {
                None
            } else {
                // 포인터를 Box로 되돌리고, 안의 값을 리턴
                let ptr = result as *mut u8 as *mut Node<T>;
                let head = Box::from_raw(ptr);
                Some((*head).data)
            }
        }
    }
}

impl<T> Drop for StackHead<T> {
    fn drop(&mut self) {
        // 데이터 삭제
        let mut node = self.head;
        while node != null_mut() {
            // 포인터를 Box로 되돌리는 조작을 반복함
            let n = unsafe { Box::from_raw(node) };
            node = n.next;
        }
    }
}

use std::cell::UnsafeCell;

// StackHead를 UnsafeCell로 저장
pub struct Stack<T> {
    data: UnsafeCell<StackHead<T>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            data: UnsafeCell::new(StackHead::new()),
        }
    }

    pub fn get_mut(&self) -> &mut StackHead<T> {
        unsafe { &mut *self.data.get() }
    }
}

// 스레드 사이의 데이터 공유 및 채널을 사용한 송수신이 가능하도록 설정
unsafe impl<T> Sync for Stack<T> {}
unsafe impl<T> Send for Stack<T> {}