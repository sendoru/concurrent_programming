use crate::semaphore::Semaphore;
use std::collections::LinkedList;
use std::sync::{Arc, Condvar, Mutex};

// 송신단을 위한 타입 ❶
#[derive(Clone)]
pub struct Sender<T> {
    sem: Arc<Semaphore>, // 유한성을 구현하는 세마포
    buf: Arc<Mutex<LinkedList<T>>>, // 큐
    cond: Arc<Condvar>, // 읽기 측의 조건 변수
}

impl<T: Send> Sender<T> { // ❷
    // 송신 함수
    pub fn send(&self, data: T) {
        self.sem.wait(); // 큐의 최댓값에 도달하면 대기 ❸
        let mut buf = self.buf.lock().unwrap();
        buf.push_back(data); // 인큐
        self.cond.notify_one(); // 읽기 측에 대한 알림 ❹
    }
}

// 수신단을 위한 타입 ❶
pub struct Receiver<T> {
    sem: Arc<Semaphore>, // 유한성을 구현하는 세마포
    buf: Arc<Mutex<LinkedList<T>>>, // 큐
    cond: Arc<Condvar>, // 읽기 측의 조건 변수
}

impl<T> Receiver<T> {
    pub fn recv(&self) -> T {
        let mut buf = self.buf.lock().unwrap();
        loop {
            // 큐에서 추출 ❷
            if let Some(data) = buf.pop_front() {
                self.sem.post(); // ❸
                return data;
            }
            // 빈 경우 대기 ❹
            buf = self.cond.wait(buf).unwrap();
        }
    }
}

pub fn channel<T>(max: isize) -> (Sender<T>, Receiver<T>) {
    assert!(max > 0);
    let sem = Arc::new(Semaphore::new(max));
    let buf = Arc::new(Mutex::new(LinkedList::new()));
    let cond = Arc::new(Condvar::new());
    let tx = Sender {
        sem: sem.clone(),
        buf: buf.clone(),
        cond: cond.clone(),
    };
    let rx = Receiver { sem, buf, cond };
    (tx, rx)
}