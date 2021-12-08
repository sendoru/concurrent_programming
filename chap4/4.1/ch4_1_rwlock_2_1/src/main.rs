use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        let _flag = val.read().unwrap(); // ❶
        *val.write().unwrap() = false; // ❷
        println!("deadlock");
    });

    t.join().unwrap();
}