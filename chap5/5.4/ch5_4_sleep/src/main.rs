// 블록되는 슬립 sleep
// use std::{thread, time}; // ❶
//
// #[tokio::main]
// async fn main() {
//     // join으로 종료 대기
//     tokio::join!(async move { // ❷
//         // 10초 슬립 ❸
//         let ten_secs = time::Duration::from_secs(10);
//         thread::sleep(ten_secs);
//     });
// }

// Tokio 함수를 가진 슬립
use std::time;

#[tokio::main]
async fn main() {
    // join으로 종료 대기
    tokio::join!(async move {
        // 10초 슬립
        let ten_secs = time::Duration::from_secs(10);
        tokio::time::sleep(ten_secs).await; // ❶
    });
}