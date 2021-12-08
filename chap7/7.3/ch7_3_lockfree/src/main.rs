#![feature(asm)]

use std::sync::Arc;

mod stack;

const NUM_LOOP: usize = 1000000; // 루프 횟수
const NUM_THREADS: usize = 4;    // 스레드 수

use stack::Stack;

fn main() {
    let stack = Arc::new(Stack::<usize>::new());
    let mut v = Vec::new();

    for i in 0..NUM_THREADS {
        let stack0 = stack.clone();
        let t = std::thread::spawn(move || {
            if i & 1 == 0 {
                // 짝수 스레드는 push
                for j in 0..NUM_LOOP {
                    let k = i * NUM_LOOP + j;
                    stack0.get_mut().push(k);
                    println!("push: {}", k);
                }
                println!("finished push: #{}", i);
            } else {
                // 홀수 스레드는 pop
                for _ in 0..NUM_LOOP {
                    loop {
                        // pop None이면 재시도
                        if let Some(k) = stack0.get_mut().pop() {
                            println!("pop: {}", k);
                            break;
                        }
                    }
                }
                println!("finished pop: #{}", i);
            }
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }

    assert!(stack.get_mut().pop() == None);
}