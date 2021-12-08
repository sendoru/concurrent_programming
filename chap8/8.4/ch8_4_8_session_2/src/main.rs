#[macro_use]
extern crate session_types;
use session_types as S; // ❶
use std::thread;
use std::collections::HashMap;

type Put = S::Recv<u64, S::Recv<u64, S::Var<S::Z>>>;
type Get = S::Recv<u64, S::Send<Option<u64>, S::Var<S::Z>>>;

type DBServer = S::Rec<S::Offer<Put, S::Offer<Get, S::Eps>>>;
type DBClient = <DBServer as S::HasDual>::Dual;

fn db_server_macro(c: S::Chan<(), DBServer>) {
    let mut c_enter = c.enter();
    let mut db = HashMap::new();

    loop {
        let c = c_enter;
        offer! {c, // ❶
            Put => { // ❷
                let (c, key) = c.recv();
                let (c, val) = c.recv();
                db.insert(key, val);
                c_enter = c.zero();
            },
            Get => {
                let (c, key) = c.recv();
                    let c = if let Some(val) = db.get(&key) {
                        c.send(Some(*val))
                    } else {
                        c.send(None)
                    };
                    c_enter = c.zero();
            },
            Quit => {
                c.close();
                return;
            }
        }
    }
}

fn db_server(c: S::Chan<(), DBServer>) {
    let mut c_enter = c.enter(); // ❶
    let mut db = HashMap::new(); // DB 데이터

    loop {
        match c_enter.offer() { // Put이 선택됨 ❷
            S::Branch::Left(c) => {
                let (c, key) = c.recv();
                let (c, val) = c.recv();
                db.insert(key, val); // DB에 데이터 삽입
                c_enter = c.zero();  // Rec로 점프 ❸
            }
            S::Branch::Right(c) => match c.offer() { // Get or 종료 ❹
                S::Branch::Left(c) => { // Get이 선택됨 ❺
                    let (c, key) = c.recv();
                    let c = if let Some(val) = db.get(&key) {
                        c.send(Some(*val))
                    } else {
                        c.send(None)
                    };
                    c_enter = c.zero(); // Rec으로 점프 ❻
                }
                S::Branch::Right(c) => { // 종료가 선택됨 ❼
                    c.close(); // 세션 클로즈 ❽
                    return;
                }
            },
        }
    }
}

fn db_client(c: S::Chan<(), DBClient>) {
    let c = c.enter(); // Rec 안으로 처리를 이행
    // Put을 2회 실시
    let c = c.sel1().send(10).send(4).zero();
    let c = c.sel1().send(50).send(7).zero();

    // Get
    let (c, val) = c.sel2().sel1().send(10).recv();
    println!("val = {:?}", val); // Some(4)

    let c = c.zero(); // Rec으로 점프

    // Get
    let (c, val) = c.sel2().sel1().send(20).recv();
    println!("val = {:?}", val); // None

    // 종료
    let _ = c.zero().sel2().sel2().close();
}

type SChan = S::Chan<(), S::Send<(), S::Eps>>; // ❶
type ChanRecv = S::Recv<SChan, S::Eps>; // ❷
type ChanSend = <ChanRecv as S::HasDual>::Dual;

fn chan_recv(c: S::Chan<(), ChanRecv>) {
    let (c, cr) = c.recv(); // 채널 엔드포인트를 수신 ❸
    c.close();
    let cr = cr.send(()); // 수신한 엔드포인트에 대해 송신 ❹
    cr.close();
}

fn chan_send(c: S::Chan<(), ChanSend>) {
    let (c1, c2) = S::session_channel(); // 채널 생성
    let c = c.send(c1); // 채널 엔드포인트를 송신❺
    c.close();
    let (c2, _) = c2.recv(); // 송신한 엔드포인트(端点)의 반대측에서 수신 ❻
    c2.close();
}

fn main() {
    let (server_chan, client_chan) = S::session_channel();
    let srv_t = thread::spawn(move || db_server(server_chan));
    let cli_t = thread::spawn(move || db_client(client_chan));
    srv_t.join().unwrap();
    cli_t.join().unwrap();

    println!("--------------------");

    // 먀크로 이용 예시
    let (server_chan, client_chan) = S::session_channel();
    let srv_t = thread::spawn(move || db_server_macro(server_chan));
    let cli_t = thread::spawn(move || db_client(client_chan));
    srv_t.join().unwrap();
    cli_t.join().unwrap();

    println!("--------------------");

    // 채널 송수신 이용 예시
    let (server_chan, client_chan) = S::session_channel();
    let srv_t = thread::spawn(move || chan_recv(server_chan));
    let cli_t = thread::spawn(move || chan_send(client_chan));
    srv_t.join().unwrap();
    cli_t.join().unwrap();
}