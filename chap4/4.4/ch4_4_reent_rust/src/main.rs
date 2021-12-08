use std::sync::{Arc, Mutex};

fn main() {
    // 뮤텍스를 Arc로 작성하고 클론
    let lock0 = Arc::new(Mutex::new(0)); // ❶
    // Arc의 클론은 참조 카운터를 증가하기만 한다
    let lock1 = lock0.clone(); // ❷

    let a = lock0.lock().unwrap();
    let b = lock1.lock().unwrap(); // 데드록 ❸
    println!("{}", a);
    println!("{}", b);
}