use std::sync::{Arc, Mutex}; // ❶
use std::thread;

fn some_func(lock: Arc<Mutex<u64>>) { // ❷
    loop {
        // 록을 하지 않으면 Mutex 타입 안의 값은 참조 불가
        let mut val = lock.lock().unwrap(); // ❸
        *val += 1;
        println!("{}", *val);
    }
}

fn main() {
    // Arc는 스레드 세이프한 참조 카운터 타입의 스마트 포인터
    let lock0 = Arc::new(Mutex::new(0)); // ❹

    // 참조 카운터가 인크리먼트될 뿐이며
    // 내용은 클론되지 않음
    let lock1 = lock0.clone(); // ❺

    // 스레드 생성
    // 클로저 내 변수로 이동
    let th0 = thread::spawn(move || { // ❻
        some_func(lock0);
    });

    // 스레드 생성
    // 클로저 내 변수로 이동
    let th1 = thread::spawn(move || {
        some_func(lock1);
    });

    // 약속
    th0.join().unwrap();
    th1.join().unwrap();
}