use std::sync::Arc;
use std::{thread, time};

mod tl2;

// 철학자 수
const NUM_PHILOSOPHERS: usize = 8;

fn philosopher(stm: Arc<tl2::STM>, n: usize) { // ❶
    // 왼쪽과 오른쪽 포크용 메모리 ❷
    let left = 8 * n;
    let right = 8 * ((n + 1) % NUM_PHILOSOPHERS);

    for _ in 0..500000 {
        // 포크를 든다
        while !stm
            .write_transaction(|tr| {
                let mut f1 = load!(tr, left);  // 왼쪽 포크 ❸
                let mut f2 = load!(tr, right); // 오른쪽 포크
                if f1[0] == 0 && f2[0] == 0 { // ❹
                    // 양쪽이 모두 비어있으면 1로 설정
                    f1[0] = 1;
                    f2[0] = 1;
                    store!(tr, left, f1);
                    store!(tr, right, f2);
                    tl2::STMResult::Ok(true)
                } else {
                    // 양쪽을 들 수 없으면 취득 실패
                    tl2::STMResult::Ok(false)
                }
            })
            .unwrap()
        { }

        // 포크를 놓는다 ❺
        stm.write_transaction(|tr| {
            let mut f1 = load!(tr, left);
            let mut f2 = load!(tr, right);
            f1[0] = 0;
            f2[0] = 0;
            store!(tr, left, f1);
            store!(tr, right, f2);
            tl2::STMResult::Ok(())
        });
    }
}

// 관측자
fn observer(stm: Arc<tl2::STM>) {
    for _ in 0..10000 {
        // 포크의 현재 상태를 얻는다 ❶
        let chopsticks = stm
            .read_transaction(|tr| {
                let mut v = [0; NUM_PHILOSOPHERS];
                for i in 0..NUM_PHILOSOPHERS {
                    v[i] = load!(tr, 8 * i)[0];
                }

                tl2::STMResult::Ok(v)
            })
            .unwrap();

        println!("{:?}", chopsticks);

        // 들고 있는 포크의 수가 홀수이면 올바르지 않음 ❷
        let mut n = 0;
        for c in &chopsticks {
            if *c == 1 {
                n += 1;
            }
        }

        if n & 1 != 0 {
            panic!("inconsistent");
        }

        // 100마이크로초 동안 슬립
        let us = time::Duration::from_micros(100);
        thread::sleep(us);
    }
}

fn main() {
    let stm = Arc::new(tl2::STM::new());
    let mut v = Vec::new();

    // 철학자 스레드 생성
    for i in 0..NUM_PHILOSOPHERS {
        let s = stm.clone();
        let th = std::thread::spawn(move || philosopher(s, i));
        v.push(th);
    }

    // 관측자 스레드 생성
    let obs = std::thread::spawn(move || observer(stm));

    for th in v {
        th.join().unwrap();
    }

    obs.join().unwrap();
}