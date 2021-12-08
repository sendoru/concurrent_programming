use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        let _ = val.read().unwrap(); // ❶
        *val.write().unwrap() = false; // ❷
        println!("not deadlock");
    });

    t.join().unwrap();
}