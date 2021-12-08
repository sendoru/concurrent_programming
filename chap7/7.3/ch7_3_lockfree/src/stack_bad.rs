use std::ptr::null_mut;
use std::sync::atomic::{AtomicPtr, Ordering};

// 스택의 노드. 리스트 구조로 관리 ❶
struct Node<T> {
    next: AtomicPtr<Node<T>>,
    data: T,
}

// 스택의 선두
pub struct StackBad<T> {
    head: AtomicPtr<Node<T>>,
}

impl<T> StackBad<T> {
    pub fn new() -> Self {
        StackBad {
            head: AtomicPtr::new(null_mut()),
        }
    }

    pub fn push(&self, v: T) { // ❷
        // 추가할 노드를 작성
        let node = Box::new(Node {
            next: AtomicPtr::new(null_mut()),
            data: v,
        });

        // Box 타입의 값으로부터 포인터를 꺼낸다
        let ptr = Box::into_raw(node);

        unsafe {
            // 아토믹하게 헤드를 업데이트 ❸
            loop {
                // head의 값을 취득
                let head = self.head.load(Ordering::Relaxed);

                // 추가할 노드의 next를 haed로 설정
                (*ptr).next.store(head, Ordering::Relaxed);

                // head의 값이 업데이트되지 않으면, 추가할 노드에 업데이트
                if let Ok(_) =
                    self.head
                        .compare_exchange_weak(
                            head, // 값이 head이면
                            ptr,  // ptr로 업데이트
                            Ordering::Release, // 성공 시 오더
                            Ordering::Relaxed  // 실패 시 오더
                ) {
                    break;
                }
            }
        }
    }

    pub fn pop(&self) -> Option<T> { // ❹
        unsafe {
            // 아토믹하게 헤드를 업데이트
            loop {
                // head의 값을 취득 ❺
                let head = self.head.load(Ordering::Relaxed);
                if head == null_mut() {
                    return None; // head가 null이면 None
                }

                // head.next를 취득 ❻
                let next = (*head).next.load(Ordering::Relaxed);

                // head의 값이 업데이트되어 있지 않으면,
                // head.next를 새로운 header로 업데이트 ❼
                if let Ok(_) = self.head.compare_exchange_weak(
                    head, // 값이 head이면
                    next, // next오 업데이트
                    Ordering::Acquire, // 성공 시 오더
                    Ordering::Relaxed, // 실패 시 오더
                ) {
                    // 포인터를 Box로 되돌리고, 안의 값을 리턴 
                    let h = Box::from_raw(head);
                    return Some((*h).data);
                }
            }
        }
    }
}

impl<T> Drop for StackBad<T> {
    fn drop(&mut self) {
        // 데이터 삭제
        let mut node = self.head.load(Ordering::Relaxed);
        while node != null_mut() {
            // 포인터를 Box로 되돌리는 조작을 반복함
            let n = unsafe { Box::from_raw(node) };
            node = n.next.load(Ordering::Relaxed)
        }
    }
}