use tokio::sync::oneshot; // ❶

// 미래 언젠가의 시점에서 값이 결정되는 함수 ❷
async fn set_val_later(tx: oneshot::Sender<i32>) {
    let ten_secs = std::time::Duration::from_secs(10);
    tokio::time::sleep(ten_secs).await;
    if let Err(_) = tx.send(100) { // ❸
        println!("failed to send");
    }
}

#[tokio::main]
pub async fn main() {
    let (tx, rx) = oneshot::channel(); // ❹

    tokio::spawn(set_val_later(tx)); // ❺

    match rx.await { // 값 읽기 ❻
        Ok(n) => {
            println!("n = {}", n);
        }
        Err(e) => {
            println!("failed to receive: {}", e);
            return;
        }
    }
}