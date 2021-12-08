use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpListener;

fn main() {
    // TCP 10000번 포트를 리스닝
    let listener = TcpListener::bind("127.0.0.1:10000").unwrap(); // ❶

    // 커넥션 요구를 받아들인다
    while let Ok((stream, _)) = listener.accept() { // ❷
        // 읽기, 쓰기 객체를 생성 ❸
        let stream0 = stream.try_clone().unwrap();
        let mut reader = BufReader::new(stream0);
        let mut writer = BufWriter::new(stream);

        // 1행씩 읽어 같은 것을 쓴다 ❹
        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();
        writer.write(buf.as_bytes()).unwrap();
        writer.flush().unwrap(); // ❺
    }
}