mod banker;

use banker::Banker;
use std::thread;

const NUM_LOOP: usize = 100000;

fn main() {
    // 이용 가능한 포크의 수, 철학자가 이용하는 포크 최대 수 설정
    let banker = Banker::<2, 2>::new([1, 1], [[1, 1], [1, 1]]);
    let banker0 = banker.clone();

    let philosopher0 = thread::spawn(move || {
        for _ in 0..NUM_LOOP {
            // 포크 0과 1을 확보
            while !banker0.take(0, 0) {}
            while !banker0.take(0, 1) {}

            println!("0: eating");

            // 포크 0과 1을 반환
            banker0.release(0, 0);
            banker0.release(0, 1);
        }
    });

    let philosopher1 = thread::spawn(move || {
        for _ in 0..NUM_LOOP {
            // 포크 1과 0을 확보
            while !banker.take(1, 1) {}
            while !banker.take(1, 0) {}

            println!("1: eating");

            // 포크 1과 0을 반환
            banker.release(1, 1);
            banker.release(1, 0);
        }
    });

    philosopher0.join().unwrap();
    philosopher1.join().unwrap();
}