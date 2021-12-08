// 블로킹 함수
fn do_block(n: u64) -> u64 {
    let ten_secs = std::time::Duration::from_secs(10);
    std::thread::sleep(ten_secs);
    n
}

// async 함수
async fn do_print() {
    let sec = std::time::Duration::from_secs(1);
    for _ in 0..20 {
        tokio::time::sleep(sec).await;
        println!("wake up");
    }
}

#[tokio::main]
pub async fn main() {
    // 블로킹 함수 호출
    let mut v = Vec::new();
    for n in 0..32 {
        let t = tokio::task::spawn_blocking(move || do_block(n)); // ❶
        v.push(t);
    }

    // async 함수 호출.
    let p = tokio::spawn(do_print()); // ❷

    for t in v {
        let n = t.await.unwrap();
        println!("finished: {}", n);
    }

    p.await.unwrap()
}