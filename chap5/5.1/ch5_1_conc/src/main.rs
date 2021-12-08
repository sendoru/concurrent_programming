use nix::sys::epoll::{
    epoll_create1, epoll_ctl, epoll_wait, EpollCreateFlags, EpollEvent, EpollFlags, EpollOp,
};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpListener;
use std::os::unix::io::{AsRawFd, RawFd};

fn main() {
    // epoll 플래그 단축 계열
    let epoll_in = EpollFlags::EPOLLIN;
    let epoll_add = EpollOp::EpollCtlAdd;
    let epoll_del = EpollOp::EpollCtlDel;

    // TCP 10000번 포트를 리슨
    let listener = TcpListener::bind("127.0.0.1:10000").unwrap();

    // epoll영 객체를 생성
    let epfd = epoll_create1(EpollCreateFlags::empty()).unwrap(); // ❶

    // 리슨용 소켓을 감시 대상에 추가 ❷
    let listen_fd = listener.as_raw_fd();
    let mut ev = EpollEvent::new(epoll_in, listen_fd as u64);
    epoll_ctl(epfd, epoll_add, listen_fd, &mut ev).unwrap();

    let mut fd2buf = HashMap::new();
    let mut events = vec![EpollEvent::empty(); 1024];

    // epoll로 이벤트 발생을 감시
    while let Ok(nfds) = epoll_wait(epfd, &mut events, -1) { // ❸
        for n in 0..nfds { // ❹
            if events[n].data() == listen_fd as u64 {
                // 리슨 소켓에 이벤트 ❺
                if let Ok((stream, _)) = listener.accept() {
                    // 읽기, 쓰기 객체를 생성
                    let fd = stream.as_raw_fd();
                    let stream0 = stream.try_clone().unwrap();
                    let reader = BufReader::new(stream0);
                    let writer = BufWriter::new(stream);

                    // fd와 reader, writer의 관계를 만듬
                    fd2buf.insert(fd, (reader, writer));

                    println!("accept: fd = {}", fd);

                    // fd를 감시 대상에 등록
                    let mut ev =
                        EpollEvent::new(epoll_in, fd as u64);
                    epoll_ctl(epfd, epoll_add,
                              fd, &mut ev).unwrap();
                }
            } else {
                // 클라이언트에서 데이터 도착 ❻
                let fd = events[n].data() as RawFd;
                let (reader, writer) =
                    fd2buf.get_mut(&fd).unwrap();

                // 1행 읽기
                let mut buf = String::new();
                let n = reader.read_line(&mut buf).unwrap();

                // 커넥션을 클로즈한 경우, epoll 감시 대상에서 재외함
                if n == 0 {
                    let mut ev =
                        EpollEvent::new(epoll_in, fd as u64);
                    epoll_ctl(epfd, epoll_del,
                              fd, &mut ev).unwrap();
                    fd2buf.remove(&fd);
                    println!("closed: fd = {}", fd);
                    continue;
                }

                print!("read: fd = {}, buf = {}", fd, buf);

                // 읽은 데이터를 그대로 쓴다
                writer.write(buf.as_bytes()).unwrap();
                writer.flush().unwrap();
            }
        }
    }
}