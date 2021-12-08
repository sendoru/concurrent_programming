use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};
use nix::{
    errno::Errno,
    sys::{
        epoll::{
            epoll_create1, epoll_ctl, epoll_wait,
            EpollCreateFlags, EpollEvent, EpollFlags, EpollOp,
        },
        eventfd::{eventfd, EfdFlags}, // eventfd용 임포트 ❶
    },
    unistd::write,
};
use std::{
    collections::{HashMap, VecDeque},
    future::Future,
    io::{BufRead, BufReader, BufWriter, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    os::unix::io::{AsRawFd, RawFd},
    pin::Pin,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::{Context, Poll, Waker},
};

fn write_eventfd(fd: RawFd, n: usize) {
    // usize를 *const u8로 변환
    let ptr = &n as *const usize as *const u8;
    let val = unsafe {
        std::slice::from_raw_parts(
            ptr, std::mem::size_of_val(&n))
    };
    // write 시스템콜 호출
    write(fd, &val).unwrap();
}

enum IOOps {
    ADD(EpollFlags, RawFd, Waker), // epoll에 추가
    REMOVE(RawFd),                 // epoll에서 삭제
}

struct IOSelector {
    wakers: Mutex<HashMap<RawFd, Waker>>, // fd에서 waker
    queue: Mutex<VecDeque<IOOps>>,        // IO 큐
    epfd: RawFd,  // epoll의 fd
    event: RawFd, // eventfd의 fd
}

impl IOSelector {
    fn new() -> Arc<Self> { // ❶
        let s = IOSelector {
            wakers: Mutex::new(HashMap::new()),
            queue: Mutex::new(VecDeque::new()),
            epfd: epoll_create1(EpollCreateFlags::empty()).unwrap(),
            // eventfd 생성
            event: eventfd(0, EfdFlags::empty()).unwrap(), // ❷
        };
        let result = Arc::new(s);
        let s = result.clone();

        // epoll용 스레드 생성 ❸
        std::thread::spawn(move || s.select());

        result
    }

    // epoll로 감시하기 위한 함수 ❹
    fn add_event(
        &self,
        flag: EpollFlags, // epoll 플래그
        fd: RawFd,        // 감시 대상 파일 디스크립터
        waker: Waker,
        wakers: &mut HashMap<RawFd, Waker>,
    ) {
        // 각 정의의 숏컷
        let epoll_add = EpollOp::EpollCtlAdd;
        let epoll_mod = EpollOp::EpollCtlMod;
        let epoll_one = EpollFlags::EPOLLONESHOT;

        // EPOLLONESHOT을 지정해, 일단 이벤트가 발생하면
        // 그 fd로의 이벤트는 재설정할 떄까지 알림이 발생하지 않게 된다 ❺
        let mut ev =
            EpollEvent::new(flag | epoll_one, fd as u64);

        // 감시 대상에 추가
        if let Err(err) = epoll_ctl(self.epfd, epoll_add, fd,
                                    &mut ev) {
            match err {
                nix::Error::Sys(Errno::EEXIST) => {
                    // 이미 추가되어있는 경우에는 재설정❻
                    epoll_ctl(self.epfd, epoll_mod, fd,
                              &mut ev).unwrap();
                }
                _ => {
                    panic!("epoll_ctl: {}", err);
                }
            }
        }

        assert!(!wakers.contains_key(&fd));
        wakers.insert(fd, waker); // ❼
    }

    // epoll의 감시해서 삭제하기 위한 함수 ❽
    fn rm_event(&self, fd: RawFd, wakers: &mut HashMap<RawFd, Waker>) {
        let epoll_del = EpollOp::EpollCtlDel;
        let mut ev = EpollEvent::new(EpollFlags::empty(),
                                     fd as u64);
        epoll_ctl(self.epfd, epoll_del, fd, &mut ev).ok();
        wakers.remove(&fd);
    }

    fn select(&self) { // ❾
        // 각 장의의 숏컷
        let epoll_in = EpollFlags::EPOLLIN;
        let epoll_add = EpollOp::EpollCtlAdd;

        // eventfd를 epoll의 감시 대상에 추가 ❿
        let mut ev = EpollEvent::new(epoll_in,
                                     self.event as u64);
        epoll_ctl(self.epfd, epoll_add, self.event,
                  &mut ev).unwrap();

        let mut events = vec![EpollEvent::empty(); 1024];
        // event 발생을 감시
        while let Ok(nfds) = epoll_wait(self.epfd, // ⓫
                                        &mut events, -1) {
            let mut t = self.wakers.lock().unwrap();
            for n in 0..nfds {
                if events[n].data() == self.event as u64 {
                    // eventfd의 경우, 추가 및 삭제 요구를 처리 ⓬
                    let mut q = self.queue.lock().unwrap();
                    while let Some(op) = q.pop_front() {
                        match op {
                            // 추가
                            IOOps::ADD(flag, fd, waker) =>
                                self.add_event(flag, fd, waker,
                                               &mut t),
                            // 삭제
                            IOOps::REMOVE(fd) =>
                                self.rm_event(fd, &mut t),
                        }
                    }
                } else {
                    // 실향 큐에 추가 ⓭
                    let data = events[n].data() as i32;
                    let waker = t.remove(&data).unwrap();
                    waker.wake_by_ref();
                }
            }
        }
    }

    // 파일 디스크립터 등록용 함수 ⓮
    fn register(&self, flags: EpollFlags, fd: RawFd, waker: Waker) {
        let mut q = self.queue.lock().unwrap();
        q.push_back(IOOps::ADD(flags, fd, waker));
        write_eventfd(self.event, 1);
    }

    // 파일 디스크립터 삭제용 함수 ⓯
    fn unregister(&self, fd: RawFd) {
        let mut q = self.queue.lock().unwrap();
        q.push_back(IOOps::REMOVE(fd));
        write_eventfd(self.event, 1);
    }
}

struct AsyncListener { // ❶
    listener: TcpListener,
    selector: Arc<IOSelector>,
}

impl AsyncListener {
    // TcpListener의 초기화 처리를 감싼 함수 ❷
    fn listen(addr: &str, selector: Arc<IOSelector>) -> AsyncListener {
        // 리슨 주소를 지정
        let listener = TcpListener::bind(addr).unwrap();

        // 논블로킹으로 지정
        listener.set_nonblocking(true).unwrap();

        AsyncListener {
            listener: listener,
            selector: selector,
        }
    }

    // 커넥션을 억셉트하기 위한 Future를 리턴 ❸
    fn accept(&self) -> Accept {
        Accept { listener: self }
    }
}

impl Drop for AsyncListener {
    fn drop(&mut self) { // ❹
        self.selector.unregister(self.listener.as_raw_fd());
    }
}

struct Accept<'a> {
    listener: &'a AsyncListener,
}

impl<'a> Future for Accept<'a> {
    // 반환값 타입
    type Output = (AsyncReader,          // 비동기 읽기 스트림
                   BufWriter<TcpStream>, // 쓰기 스트림
                   SocketAddr);          // 주소

    fn poll(self: Pin<&mut Self>,
            cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 억셉트를 논블로킹으로 실행
        match self.listener.listener.accept() { // ❶
            Ok((stream, addr)) => {
                // 억셉트한 경우는 
                // 읽기와 쓰기용 객체 및 주소를 리턴 ❷
                let stream0 = stream.try_clone().unwrap();
                Poll::Ready((
                    AsyncReader::new(stream0, self.listener.selector.clone()),
                    BufWriter::new(stream),
                    addr,
                ))
            }
            Err(err) => {
                // 억세스할 커넥션이 없는 경우는 epoll에 등록 ❸
                if err.kind() == std::io::ErrorKind::WouldBlock {
                    self.listener.selector.register(
                        EpollFlags::EPOLLIN,
                        self.listener.listener.as_raw_fd(),
                        cx.waker().clone(),
                    );
                    Poll::Pending
                } else {
                    panic!("accept: {}", err);
                }
            }
        }
    }
}

struct AsyncReader {
    fd: RawFd,
    reader: BufReader<TcpStream>,
    selector: Arc<IOSelector>,
}

impl AsyncReader {
    fn new(stream: TcpStream,
           selector: Arc<IOSelector>) -> AsyncReader {
        // 논블로킹으로 설정
        stream.set_nonblocking(true).unwrap();
        AsyncReader {
            fd: stream.as_raw_fd(),
            reader: BufReader::new(stream),
            selector: selector,
        }
    }

    // 1행 읽기만 하므로 Future를 리턴
    fn read_line(&mut self) -> ReadLine {
        ReadLine { reader: self }
    }
}

impl Drop for AsyncReader {
    fn drop(&mut self) {
        self.selector.unregister(self.fd);
    }
}

struct ReadLine<'a> {
    reader: &'a mut AsyncReader,
}

impl<'a> Future for ReadLine<'a> {
    // 반환값의 타입
    type Output = Option<String>;

    fn poll(mut self: Pin<&mut Self>,
            cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut line = String::new();
        // 비동기 읽기
        match self.reader.reader.read_line(&mut line) { // ❶
            Ok(0) => Poll::Ready(None),  // 커넥션 유실
            Ok(_) => Poll::Ready(Some(line)), // 1행 읽기 성공
            Err(err) => {
                // 읽을 수 없는 경우는 epoll에 등록 ❷
                if err.kind() == std::io::ErrorKind::WouldBlock {
                    self.reader.selector.register(
                        EpollFlags::EPOLLIN,
                        self.reader.fd,
                        cx.waker().clone(),
                    );
                    Poll::Pending
                } else {
                    Poll::Ready(None)
                }
            }
        }
    }
}

struct Task {
    // 실행할 코루틴
    future: Mutex<BoxFuture<'static, ()>>, // ❶
    // Executor로 스케줄링 하기 위한 채널
    sender: SyncSender<Arc<Task>>, // ❷
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) { // ❸
        // 자신을 스케줄링
        let self0 = arc_self.clone();
        arc_self.sender.send(self0).unwrap();
    }
}

struct Executor { // ❶
    // 실행 큐
    sender: SyncSender<Arc<Task>>,
    receiver: Receiver<Arc<Task>>,
}

impl Executor {
    fn new() -> Self {
        // 채널 생성. 큐의 크기는 최대 1024개
        let (sender, receiver) = sync_channel(1024);
        Executor {
            sender: sender.clone(),
            receiver,
        }
    }

    // 새롭게 Task를 생성하기 위한 Spawner를 작성 ❷
    fn get_spawner(&self) -> Spawner {
        Spawner {
            sender: self.sender.clone(),
        }
    }

    fn run(&self) { // ❸
        // 채널에서 Task를 수신해 순서대로 실행
        while let Ok(task) = self.receiver.recv() {
            // 컨텍스트를 생성
            let mut future = task.future.lock().unwrap();
            let waker = waker_ref(&task);
            let mut ctx = Context::from_waker(&waker);
            // poll을 호출하고 실행
            let _ = future.as_mut().poll(&mut ctx);
        }
    }
}

struct Spawner { // ❶
    sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) { // ❷
        let future = future.boxed();    // Future를 Box화
        let task = Arc::new(Task {      // Task 생성
            future: Mutex::new(future),
            sender: self.sender.clone(),
        });

        // 실행 큐에 인큐
        self.sender.send(task).unwrap();
    }
}

fn main() {
    let executor = Executor::new();
    let selector = IOSelector::new();
    let spawner = executor.get_spawner();

    let server = async move { // ❶
        // 비동기 억셉트용 리스너 생성 ❷
        let listener = AsyncListener::listen("127.0.0.1:10000",
                                             selector.clone());
        loop {
            // 비동기 커넥션 억셉트 ❸
            let (mut reader, mut writer, addr) =
                listener.accept().await;
            println!("accept: {}", addr);

            // 커넥션 별로 태스크 생성 ❹
            spawner.spawn(async move {
                // 1헹 비동기 읽기 ❺
                while let Some(buf) = reader.read_line().await {
                    print!("read: {}, {}", addr, buf);
                    writer.write(buf.as_bytes()).unwrap();
                    writer.flush().unwrap();
                }
                println!("close: {}", addr);
            });
        }
    };

    // 태스크를 생성하고 실행
    executor.get_spawner().spawn(server);
    executor.run();
}