use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{fence, AtomicBool, AtomicUsize, Ordering};

// 스레드의 최대 수
pub const NUM_LOCK: usize = 8; // ❶

// NUM_LOCK의 여분을 구하기 위한 비트 마스크
const MASK: usize = NUM_LOCK - 1; // ❷

// 공평한 록용 타입 ❸
pub struct FairLock<T> {
    waiting: Vec<AtomicBool>, // 록 획등 중인 스레드
    lock: AtomicBool,         // 록용 변수
    turn: AtomicUsize,        // 록 획득 우선 스레드
    data: UnsafeCell<T>,      // 보호 대상 데이터
}

// 록 해제, 보호 대상 데이터로의 접근을 수행하기 위한 타입 ❹
pub struct FairLockGuard<'a, T> {
    fair_lock: &'a FairLock<T>,
    idx: usize, // 스레드 번호
}

impl<T> FairLock<T> {
    pub fn new(v: T) -> Self { // ❶
        let mut vec = Vec::new();
        for _ in 0..NUM_LOCK {
            vec.push(AtomicBool::new(false));
        }

        FairLock {
            waiting: vec,
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(v),
            turn: AtomicUsize::new(0),
        }
    }

    // 록 함수 ❷
    // idx는 스레드 번호
    pub fn lock(&self, idx: usize) -> FairLockGuard<T> {
        assert!(idx < NUM_LOCK); // idx가 최대 수 미만인지 검사 ❸

        // 자신의 스레드를 록 획득 시행 중으로 설정 
        self.waiting[idx].store(true, Ordering::Relaxed); // ❹
        loop {
            // 다른 스레드가 false를 설정한 경우 록 획득❺
            if !self.waiting[idx].load(Ordering::Relaxed) {
                break;
            }

            // 공유 변수를 이용해 록 획득을 테스트❻
            if !self.lock.load(Ordering::Relaxed) {
                if let Ok(_) = self.lock.compare_exchange_weak(
                    false, // false이면
                    true,  // true를 써넣음
                    Ordering::Relaxed, // 성공 시 오더
                    Ordering::Relaxed, // 실패 시 오더
                ) {
                    break; // 록 획득
                }
            }
        }
        fence(Ordering::Acquire);

        FairLockGuard {
            fair_lock: self,
            idx: idx,
        }
    }
}

// 록 획득 후에 자동으로 해제되도록 Drop 트레이트를 구현 ❶
impl<'a, T> Drop for FairLockGuard<'a, T> {
    fn drop(&mut self) {
        let fl = self.fair_lock; // fair_lock으로의 참조를 획득

        // 자신의 스레드를 록 획득 시도 중이 아닌 상태로 설정❷
        fl.waiting[self.idx].store(false, Ordering::Relaxed);

        // 현재의 록 획득 우선 스레드가 자신이라면 다음 스레드로 설정 ❸
        let turn = fl.turn.load(Ordering::Relaxed);
        let next = if turn == self.idx {
            (turn + 1) & MASK
        } else {
            turn
        };

        if fl.waiting[next].load(Ordering::Relaxed) { // ❹
            // 다음 록 획득 우선 스레드가 록 획득 중이면
            // 해당 스레드에 록을 전달한다
            fl.turn.store(next, Ordering::Relaxed);
            fl.waiting[next].store(false, Ordering::Release);
        } else {
            // 다음 록 획득 우선 스레드가 록 획득 중이 아니면
            // 그 다음 스레드를 록 획득 우선 스레드로 설정하고 록을 해제한다
            fl.turn.store((next + 1) & MASK, Ordering::Relaxed);
            fl.lock.store(false, Ordering::Release);
        }
    }
}

// FairLock 타입은 스레드 사이에서 공유 가능하도록 설정
unsafe impl<T> Sync for FairLock<T> {}
unsafe impl<T> Send for FairLock<T> {}

// 보호 대상 데이터의 이뮤터블한 참조 제외
impl<'a, T> Deref for FairLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.fair_lock.data.get() }
    }
}

// 보호 대상 데이터의 뮤터블한 참조 제외
impl<'a, T> DerefMut for FairLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.fair_lock.data.get() }
    }
}