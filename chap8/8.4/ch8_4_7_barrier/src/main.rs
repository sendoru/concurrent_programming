use std::sync::mpsc::{channel, Sender}; // ❶

fn main() {
    let mut v = Vec::new();

    // 채널을 작성 ❷
    let (tx, rx) = channel::<Sender<()>>();

    // 배리어 동기용 스레드 ❸
    let barrier = move || {
        let x = rx.recv().unwrap();
        let y = rx.recv().unwrap();
        let z = rx.recv().unwrap();
        println!("send!");
        x.send(()).unwrap();
        y.send(()).unwrap();
        z.send(()).unwrap();
    };
    let t = std::thread::spawn(barrier);
    v.push(t);

    // 클라이언트 스레드 ❹
    for _ in 0..3 {
        let tx_c = tx.clone(); // ❺
        let node = move || {
            // 배리어 동기 ❻
            let (tx0, rx0) = channel();
            tx_c.send(tx0).unwrap();
            rx0.recv().unwrap();
            println!("received!");
        };
        let t = std::thread::spawn(node);
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }
}