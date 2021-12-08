use tokio::io::{AsyncBufReadExt, AsyncWriteExt}; // ❶
use tokio::io;
use tokio::net::TcpListener; // ❷

#[tokio::main] // ❸
async fn main() -> io::Result<()> {
    // 10000번 포트에서 TCP 리슨 ❹
    let listener = TcpListener::bind("127.0.0.1:10000").await.unwrap();

    loop {
        // TCP 커넥트 억셉트 ❺
        let (mut socket, addr) = listener.accept().await?;
        println!("accept: {}", addr);

        // 비동기 태스크 생성 ❻
        tokio::spawn(async move {
            // 버퍼 읽기 쓰기용 객체 생성 ❼
            let (r, w) = socket.split(); // ❽
            let mut reader = io::BufReader::new(r);
            let mut writer = io::BufWriter::new(w);

            let mut line = String::new();
            loop {
                line.clear(); // ❾
                match reader.read_line(&mut line).await { // ❿
                    Ok(0) => { // 커넥션 로스
                        println!("closed: {}", addr);
                        return;
                    }
                    Ok(_) => {
                        print!("read: {}, {}", addr, line);
                        writer.write_all(line.as_bytes()).await.unwrap();
                        writer.flush().await.unwrap();
                    }
                    Err(e) => { // 에러ー
                        println!("error: {}, {}", addr, e);
                        return;
                    }
                }
            }
        });
    }
}