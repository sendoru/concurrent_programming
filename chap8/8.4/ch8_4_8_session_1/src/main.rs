extern crate session_types;
use session_types as S; // ❶
use std::thread;

type Client = S::Send<u64, S::Choose<S::Recv<u64, S::Eps>, S::Recv<bool, S::Eps>>>; // 클라이언트의 엔드포인트 타입 ❷
type Server = <Client as S::HasDual>::Dual; // 서버의 엔드포인트 타입 ❸

enum Op {
    Square, // 2제곱 명령
    Even,   // 짝수 판정 명령
}

fn server(c: S::Chan<(), Server>) {
    let (c, n) = c.recv(); // 데이터 수신 ❶
    match c.offer() {
        S::Branch::Left(c) => { // 2제곱 명령 ❷
            c.send(n * n).close(); // ❸
        }
        S::Branch::Right(c) => { // 짝수 판정 명령 ❹
            c.send(n & 1 == 0).close(); // ❺
        }
    }
}

fn client(c: S::Chan<(), Client>, n: u64, op: Op) {
    let c = c.send(n); // ❶
    match op {
        Op::Square => {
            let c = c.sel1();        // 1번째 선택지를 선택 ❷
            let (c, val) = c.recv(); // 데이터 수신 ❸
            c.close();               // 세션 종료 ❹
            println!("{}^2 = {}", n, val);
        }
        Op::Even => {
            let c = c.sel2();        // 2번째 선택지를 선택 ❺
            let (c, val) = c.recv(); // 데이터 수신 ❻
            c.close();               // 세션 종료 ❼
            if val {
                println!("{} is even", n);
            } else {
                println!("{} is odd", n);
            }
        }
    };
}

fn main() {
    // Even 예시
    let (server_chan, client_chan) = S::session_channel();
    let srv_t = thread::spawn(move || server(server_chan));
    let cli_t = thread::spawn(move || client(client_chan, 11, Op::Even));
    srv_t.join().unwrap();
    cli_t.join().unwrap();

    // Square 예시
    let (server_chan, client_chan) = S::session_channel();
    let srv_t = thread::spawn(move || server(server_chan));
    let cli_t = thread::spawn(move || client(client_chan, 11, Op::Square));
    srv_t.join().unwrap();
    cli_t.join().unwrap();
}