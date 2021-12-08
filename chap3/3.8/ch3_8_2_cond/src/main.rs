use std::sync::{Arc, Mutex, Condvar}; // ❶
use std::thread;

// Condvar 타입의 변수가 조건 변수이며
// Mutex와 Condvar를 포함하는 튜플이 Arc에 포함되어 전달된다
fn child(id: u64, p: Arc<(Mutex<bool>, Condvar)>) { // ❷
    let &(ref lock, ref cvar) = &*p;

    // 먼저, 뮤텍스 록을 수행한다
    let mut started = lock.lock().unwrap(); // ❸
    while !*started { // Mutex 안의 공유 변수가 false인 동안 루프 
        // wait로 대기
        started = cvar.wait(started).unwrap(); // ❹

    }

    // 다음과 같이 wait_while을 사용할 수도 있음
    // cvar.wait_while(started, |started| !*started).unwrap();

    println!("child {}", id);
}

fn parent(p: Arc<(Mutex<bool>, Condvar)>) { // ❺
    let &(ref lock, ref cvar) = &*p;

    // 먼저 뮤텍스 록을 수행한다 ❻
    let mut started = lock.lock().unwrap();
    *started = true;   // 공유 변수 업데이트
    cvar.notify_all(); // 알림
    println!("parent");
}

fn main() {
    // 뮤텍스와 조건 변수를 작성
    let pair0 = Arc::new((Mutex::new(false), Condvar::new()));
    let pair1 = pair0.clone();
    let pair2 = pair0.clone();

    let c0 = thread::spawn(move || { child(0, pair0) });
    let c1 = thread::spawn(move || { child(1, pair1) });
    let p  = thread::spawn(move || { parent(pair2) });

    c0.join().unwrap();
    c1.join().unwrap();
    p.join().unwrap();
}