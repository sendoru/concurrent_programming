use futures::future::{BoxFuture, FutureExt};
use futures::task::{waker_ref, ArcWake};
use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender}; // ❶
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

struct Hello { // ❶
    state: StateHello,
}

// 상태 ❷
enum StateHello {
    HELLO,
    WORLD,
    END,
}

impl Hello {
    fn new() -> Self {
        Hello {
            state: StateHello::HELLO, // 초기 상태
        }
    }
}

impl Future for Hello {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        match (*self).state {
            StateHello::HELLO => {
                print!("Hello, ");
                (*self).state = StateHello::WORLD;
                cx.waker().wake_by_ref(); // 자신을 실행 큐에 인큐
                return Poll::Pending;
            }
            StateHello::WORLD => {
                println!("World!");
                (*self).state = StateHello::END;
                cx.waker().wake_by_ref(); // 자신을 실행 큐에 인큐
                return Poll::Pending;
            }
            StateHello::END => {
                return Poll::Ready(());
            }
        }
    }
}

struct Task {
    // 실행하는 코루틴
    future: Mutex<BoxFuture<'static, ()>>, // ❶
    // Executor에 스케줄링하기 위한 채널
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
        // 채널 생성. 큐의 사이즈는 최대 1024개
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
        // 채널에서 Task를 수신하고 순서대로 실행
        while let Ok(task) = self.receiver.recv() {
            // 컨텍스트를 생성
            let mut future = task.future.lock().unwrap();
            let waker = waker_ref(&task);
            let mut ctx = Context::from_waker(&waker);
            // poll을 호출해서 실행
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
    executor.get_spawner().spawn(Hello::new());
    executor.run();
}

// 5.3.1에서 나타낸 것처럼, 다음처럼 실행도 가능
// fn main() {
//     let executor = Executor::new();
//     // async로 Future 트레이트를 구현한 타입의 값으로 변환
//     executor.get_spawner().spawn(async {
//         let h = Hello::new();
//         h.await; // poll을 호출해서 실행
//     });
//     executor.run();
// }