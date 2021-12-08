use std::sync::RwLock; // ❶

fn main() {
    let lock = RwLock::new(10); // ❷
    {
        // 이뮤터블한 참조를 얻음 ❸
        let v1 = lock.read().unwrap();
        let v2 = lock.read().unwrap();
        println!("v1 = {}", v1);
        println!("v2 = {}", v2);
    }

    {
        // mutable한 참조를 얻음❹
        let mut v = lock.write().unwrap();
        *v = 7;
        println!("v = {}", v);
    }
}