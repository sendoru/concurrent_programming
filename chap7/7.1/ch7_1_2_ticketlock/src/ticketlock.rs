use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{fence, AtomicUsize, Ordering};

// 티켓록용 타입
pub struct TicketLock<T> {
    ticket: AtomicUsize, // 티켓
    turn: AtomicUsize,   // 실행 가능한 티켓
    data: UnsafeCell<T>,
}

// 록 해제, 보호 대상 데이터로의 접근을 수행하기 위한 타입
pub struct TicketLockGuard<'a, T> {
    ticket_lock: &'a TicketLock<T>,
}

impl<T> TicketLock<T> {
    pub fn new(v: T) -> Self {
        TicketLock {
            ticket: AtomicUsize::new(0),
            turn: AtomicUsize::new(0),
            data: UnsafeCell::new(v),
        }
    }

    // 록용 함수 ❶
    pub fn lock(&self) -> TicketLockGuard<T> {
        // 티켓을 취득
        let t = self.ticket.fetch_add(1, Ordering::Relaxed);
        // 소유하는 티켓의 순서가 될 때까지 스핀
        while self.turn.load(Ordering::Relaxed) != t {}
        fence(Ordering::Acquire);

        TicketLockGuard { ticket_lock: self }
    }
}

// 록 획득 후에 자동으로 해제되도록 Drop 트레이트를 구현 ❷
impl<'a, T> Drop for TicketLockGuard<'a, T> {
    fn drop(&mut self) {
        // 다음 티켓을 실행 가능하도록 설정
        self.ticket_lock.turn.fetch_add(1, Ordering::Release);
    }
}

// TicketLock 타입은 스레드 사이에서 공유 가능하도록 설정
unsafe impl<T> Sync for TicketLock<T> {}
unsafe impl<T> Send for TicketLock<T> {}

// 보호 대상 데이터의 이뮤터블한 참조 제외
impl<'a, T> Deref for TicketLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ticket_lock.data.get() }
    }
}

// 보호 대상 데이터의 뮤터블한 참조 제외
impl<'a, T> DerefMut for TicketLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ticket_lock.data.get() }
    }
}