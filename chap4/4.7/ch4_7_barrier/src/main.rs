use std::cell::UnsafeCell; // ❶
use std::ops::{Deref, DerefMut}; // ❷
use std::sync::atomic::{AtomicBool, Ordering}; // ❸
use std::sync::Arc;

const NUM_THREADS: usize = 4;
const NUM_LOOP: usize = 100000;

// 스핀록용 타입 ❹
struct SpinLock<T> {
    lock: AtomicBool,    // 록용 공유 변수
    data: UnsafeCell<T>, // 보호 대상 데이터
}

// 록 해제 및록 중에 보호 대상 데이터를 조작하기 위한 타입 ❺
struct SpinLockGuard<'a, T> {
    spin_lock: &'a SpinLock<T>,
}

impl<T> SpinLock<T> {
    fn new(v: T) -> Self {
        SpinLock {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(v),
        }
    }

    // 록 함수 ❻
    fn lock(&self) -> SpinLockGuard<T> {
        loop {
            // 록용 공유 변수가 false가 될 때까지 대기
            while self.lock.load(Ordering::Relaxed) {}

            // 록용 공유 변수를 아토믹하게 씀
            if let Ok(_) =
                self.lock
                    .compare_exchange_weak(
                        false, // false이면
                        true,  // true를 쓴다
                        Ordering::Acquire, // 성공 시의 오더
                        Ordering::Relaxed) // 실패 시의 오더
            {
                break;
            }
        }
        SpinLockGuard { spin_lock: self } // ❼
    }
}

// SpinLock 타입은 스레드 사이에서 공유 가능하도록 지정
unsafe impl<T> Sync for SpinLock<T> {} // ❽
unsafe impl<T> Send for SpinLock<T> {} // ❾

// 록 획득 후에 자동으로 해제되도록 Drop 트레이트를 구현 ❿
impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.spin_lock.lock.store(false, Ordering::Release);
    }
}

// 보호 대상 데이터의 이뮤터블한 참조 제외 ⓫
impl<'a, T> Deref for SpinLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.spin_lock.data.get() }
    }
}

// 보호 대상 데이터의 뮤터블한 참조 제외 ⓬
impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.spin_lock.data.get() }
    }
}

fn main() {
    let lock = Arc::new(SpinLock::new(0));
    let mut v = Vec::new();

    for _ in 0..NUM_THREADS {
        let lock0 = lock.clone();
        // 스레드 생성
        let t = std::thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                // 록
                let mut data = lock0.lock();
                *data += 1;
            }
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }

    println!(
        "COUNT = {} (expected = {})",
        *lock.lock(),
        NUM_LOOP * NUM_THREADS
    );
}