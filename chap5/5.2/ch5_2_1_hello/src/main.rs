use futures::future::{BoxFuture, FutureExt};
use futures::task::{waker_ref, ArcWake};
use std::future::Future;
use std::pin::Pin;
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

    // 실행 함수 ❸
    fn poll(mut self: Pin<&mut Self>,
            _cx: &mut Context<'_>) -> Poll<()> {
        match (*self).state {
            StateHello::HELLO => {
                print!("Hello, ");
                // WORLD 상태로 전이
                (*self).state = StateHello::WORLD;
                Poll::Pending // 다시 호출 가능
            }
            StateHello::WORLD => {
                println!("World!");
                // END状態に遷移
                (*self).state = StateHello::END;
                Poll::Pending // 다시 호출 가능
            }
            StateHello::END => {
                Poll::Ready(()) // 종료
            }
        }
    }
}

// 실행 단위 ❶
struct Task {
    hello: Mutex<BoxFuture<'static, ()>>,
}

impl Task {
    fn new() -> Self {
        let hello = Hello::new();
        Task {
            hello: Mutex::new(hello.boxed()),
        }
    }
}

// 아무것도 하지 않음
impl ArcWake for Task {
    fn wake_by_ref(_arc_self: &Arc<Self>) {}
}

fn main() {
    // 초기화
    let task = Arc::new(Task::new());
    let waker = waker_ref(&task);
    let mut ctx = Context::from_waker(&waker); // ❷
    let mut hello = task.hello.lock().unwrap();

    // 정지와 재개의 반복 ❸
    hello.as_mut().poll(&mut ctx);
    hello.as_mut().poll(&mut ctx);
    hello.as_mut().poll(&mut ctx);
}