use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::ptr::null_mut;
use std::sync::atomic::{fence, AtomicBool, AtomicPtr, Ordering};

pub struct MCSLock<T> { // ❶
    last: AtomicPtr<MCSNode<T>>, // 큐의 맨 마지막
    data: UnsafeCell<T>,         // 보호 대상 데이터
}

pub struct MCSNode<T> { // ❷
    next: AtomicPtr<MCSNode<T>>, // 다음 노드
    locked: AtomicBool,          // true이면 록 획득 중
}

pub struct MCSLockGuard<'a, T> {
    node: &'a mut MCSNode<T>, // 자신의 스레드 노드
    mcs_lock: &'a MCSLock<T>, // 큐의 가장 마지막과 보호 대상 데이터로의 참조
}

// 스레드끼리의 데이터 공유, 및 채널을 이용한 송수신 가능 설정
unsafe impl<T> Sync for MCSLock<T> {}
unsafe impl<T> Send for MCSLock<T> {}

impl<T> MCSNode<T> {
    pub fn new() -> Self {
        MCSNode { // MCSNodeの初期化
            next: AtomicPtr::new(null_mut()),
            locked: AtomicBool::new(false),
        }
    }
}

// 보호 대상 데이터의 이뮤터블한 참조 제외
impl<'a, T> Deref for MCSLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mcs_lock.data.get() }
    }
}

// 보호 대상 데이터의 뮤터블한 참조 제외
impl<'a, T> DerefMut for MCSLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mcs_lock.data.get() }
    }
}

impl<T> MCSLock<T> {
    pub fn new(v: T) -> Self {
        MCSLock {
            last: AtomicPtr::new(null_mut()),
            data: UnsafeCell::new(v),
        }
    }

    pub fn lock<'a>(&'a self, node: &'a mut MCSNode<T>) -> MCSLockGuard<T> {
        // 자기 스레드용 노드를 초기화 ❶
        node.next = AtomicPtr::new(null_mut());
        node.locked = AtomicBool::new(false);

        let guard = MCSLockGuard {
            node,
            mcs_lock: self,
        };

        // 자신을 큐의 맨 마지막으로 한다 ❷
        let ptr = guard.node as *mut MCSNode<T>;
        let prev = self.last.swap(ptr, Ordering::Relaxed);

        // 맨 마지막이 null이면 나무도 록을 획득하려 하지 않는 것이므로 록을 획득
        // null이 아닌 경우에는 자신을 큐의 맨 끝에 추가
        if prev != null_mut() { // ❸
            // 록 획득 중이라고 설정
            guard.node.locked.store(true, Ordering::Relaxed); // ❹

            // 자신을 큐의 맨 끝에 추가 ❺
            let prev = unsafe { &*prev };
            prev.next.store(ptr, Ordering::Relaxed);

            // 다른 스레드로부터 false로 설정될 때까지 스핀 >❻
            while guard.node.locked.load(Ordering::Relaxed) {}
        }

        fence(Ordering::Acquire);
        guard
    }
}

impl<'a, T> Drop for MCSLockGuard<'a, T> {
    fn drop(&mut self) {
        // 자신의 다음 노트가 null이고 자신이 맨 끝의 노드이면 , 맨 끝을 null로 설정한다 ❶
        if self.node.next.load(Ordering::Relaxed) == null_mut() {
            let ptr = self.node as *mut MCSNode<T>;
            if let Ok(_) = self.mcs_lock.last.compare_exchange( // ❷
                ptr,
                null_mut(),
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                return;
            }
        }

        // 자신의 다음 스레드가 lock 함수 실행 중이므로, 종료될 떄까지 대기한다 ❸
        while self.node.next.load(Ordering::Relaxed) == null_mut() {}

        // 자신의 다음 스레드를 실행 가능하게 설정 ❹
        let next = unsafe { &mut *self.node.next.load(Ordering::Relaxed) };
        next.locked.store(false, Ordering::Release);
    }
}