use std::{sync::Arc, time};
use tokio::sync::Mutex;

const NUM_TASKS: usize = 8;

// 록만 하는 태스크 ❶
async fn lock_only(v: Arc<Mutex<u64>>) {
    let mut n = v.lock().await;
    *n += 1;
}

// 록 상태에서 await를 수행하는 태스크 ❷
async fn lock_sleep(v: Arc<Mutex<u64>>) {
    let mut n = v.lock().await;
    let ten_secs = time::Duration::from_secs(10);
    tokio::time::sleep(ten_secs).await; // ❸
    *n += 1;
}

#[tokio::main]
async fn main() -> Result<(), tokio::task::JoinError> {
    let val = Arc::new(Mutex::new(0));
    let mut v = Vec::new();

    // lock_sleep 태스크 생성
    let t = tokio::spawn(lock_sleep(val.clone()));
    v.push(t);

    for _ in 0..NUM_TASKS {
        let n = val.clone();
        let t = tokio::spawn(lock_only(n)); // lock_only 태스크 생성
        v.push(t);
    }

    for i in v {
        i.await?;
    }
    Ok(())
}