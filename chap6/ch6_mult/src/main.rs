mod green;

fn mash() {
    green::spawn(ortega, 2 * 1024 * 1024);
    for _ in 0..10 {
        println!("Mash!");
        green::schedule();
    }
}

fn ortega() {
    for _ in 0..10 {
        println!("Ortega!");
        green::schedule();
    }
}

fn gaia() {
    green::spawn(mash, 2 * 1024 * 1024);
    for _ in 0..10 {
        println!("Gaia!");
        green::schedule();
    }
}

fn producer() { // ❶
    let id = green::spawn(consumer, 2 * 1024 * 1024);
    for i in 0..10 {
        green::send(id, i);
    }
}

fn consumer() { // ❷
    for _ in 0..10 {
        let msg = green::recv().unwrap();
        println!("received: count = {}", msg);
    }
}

fn main() {
    // 6.2 협조적 그린 스레드 구현 실행 예
    green::spawn_from_main(gaia, 2 * 1024 * 1024);

    println!("--------------------");

    // 6.3 액터 모델 실행 예
    green::spawn_from_main(producer, 2 * 1024 * 1024); // ❸
}