use nix::sys::mman::{mprotect, ProtFlags};
use rand;
use std::alloc::{alloc, dealloc, Layout};
use std::collections::{HashMap, HashSet, LinkedList};
use std::ffi::c_void;
use std::ptr;

// 모든 스레드 종료 시 돌아올 위치 ❶
static mut CTX_MAIN: Option<Box<Registers>> = None;

// 불필요한 스택 영역 ❷
static mut UNUSED_STACK: (*mut u8, Layout) = (ptr::null_mut(), Layout::new::<u8>());

// 스레드 실행 큐 ❸
static mut CONTEXTS: LinkedList<Box<Context>> = LinkedList::new();

// 스레드 ID 집합 ❹
static mut ID: *mut HashSet<u64> = ptr::null_mut();

// 메시지 큐 ❶
static mut MESSAGES: *mut MappedList<u64> = ptr::null_mut();

// 대기 스레드 집합❷
static mut WAITING: *mut HashMap<u64, Box<Context>> = ptr::null_mut();

#[repr(C)] // ❶
struct Registers { // ❷
    // callee 저장 레지스터
     d8: u64,  d9: u64, d10: u64, d11: u64, d12: u64,
    d13: u64, d14: u64, d15: u64, x19: u64, x20: u64,
    x21: u64, x22: u64, x23: u64, x24: u64, x25: u64,
    x26: u64, x27: u64, x28: u64,

    x30: u64, // 링크 레지스터
    sp: u64,  // 스택 레지스터
}

impl Registers {
    fn new(sp: u64) -> Self { // ❸
        Registers {
             d8: 0,  d9: 0, d10: 0, d11: 0, d12: 0,
            d13: 0, d14: 0, d15: 0, x19: 0, x20: 0,
            x21: 0, x22: 0, x23: 0, x24: 0, x25: 0,
            x26: 0, x27: 0, x28: 0,
            x30: entry_point as u64, // ❹
            sp,
        }
    }
}

extern "C" {
    fn set_context(ctx: *mut Registers) -> u64;
    fn switch_context(ctx: *const Registers) -> !;
}

// 스레드 개시 시 실행하는 함수 타입
type Entry = fn(); // ❶

// 페이지 크기. Linux에서는 4KiB
const PAGE_SIZE: usize = 4 * 1024; // 4KiB ❷

struct MappedList<T> { // ❶
    map: HashMap<u64, LinkedList<T>>,
}

impl<T> MappedList<T> {
    fn new() -> Self {
        MappedList {
            map: HashMap::new(),
        }
    }

    // key에 대응하는 리스트의 가장 마지막에 추가 ❷
    fn push_back(&mut self, key: u64, val: T) {
        if let Some(list) = self.map.get_mut(&key) {
            // 대응하는 리스트가 존재하면 추가
            list.push_back(val);
        } else {
            // 존재하지 않는 경우, 새롭게 리스트를 작성하고 추가
            let mut list = LinkedList::new();
            list.push_back(val);
            self.map.insert(key, list);
        }
    }

    // key에 대응하는 리스트의 가장 처음부터 꺼낸다 ❸
    fn pop_front(&mut self, key: u64) -> Option<T> {
        if let Some(list) = self.map.get_mut(&key) {
            let val = list.pop_front();
            if list.len() == 0 {
                self.map.remove(&key);
            }
            val
        } else {
            None
        }
    }

    fn clear(&mut self) {
        self.map.clear();
    }
}

// 컨텍스트 ❸
struct Context {
    regs: Registers,      // 레지스터
    stack: *mut u8,       // 스택
    stack_layout: Layout, // 스택 레이아웃
    entry: Entry,         // 엔트리포인트
    id: u64,              // 스레드 ID
}

impl Context {
    // 레지스터 정보로의 포인터 취득
    fn get_regs_mut(&mut self) -> *mut Registers {
        &mut self.regs as *mut Registers
    }

    fn get_regs(&self) -> *const Registers {
        &self.regs as *const Registers
    }

    fn new(func: Entry, stack_size: usize, id: u64) -> Self { // ❹
        // 스택 영역 확보 ❺
        let layout = Layout::from_size_align(stack_size, PAGE_SIZE).unwrap();
        let stack = unsafe { alloc(layout) };

        // 가드 페이지 설정 ❻
        unsafe { mprotect(stack as *mut c_void, PAGE_SIZE, ProtFlags::PROT_NONE).unwrap() };

        // 레지스터 초기화 ❼
        let regs = Registers::new(stack as u64 + stack_size as u64);

        // 컨텍스트 초기화
        Context {
            regs: regs,
            stack: stack,
            stack_layout: layout,
            entry: func,
            id: id,
        }
    }
}

fn get_id() -> u64 {
    loop {
        let rnd = rand::random::<u64>(); // ❶
        unsafe {
            if !(*ID).contains(&rnd) { // ❷
                (*ID).insert(rnd); // ❸
                return rnd;
            };
        }
    }
}

pub fn spawn(func: Entry, stack_size: usize) -> u64 { // ❶
    unsafe {
        let id = get_id(); // ❷
        CONTEXTS.push_back(Box::new(Context::new(func, stack_size, id))); // ❸
        schedule(); // ❹
        id // ❺
    }
}

pub fn schedule() {
    unsafe {
        // 실행 가능한 프로세스가 자신뿐이므로 즉시 리턴 ❶
        if CONTEXTS.len() == 1 {
            return;
        }

        // 자신의 컨텍스트를 실행 큐의 맨 끝으로 이동
        let mut ctx = CONTEXTS.pop_front().unwrap(); // ❷
        // レジスタ保存領域へのポインタを取得 ❸
        let regs = ctx.get_regs_mut();
        CONTEXTS.push_back(ctx);

        // 레지스터를 보존 ❹
        if set_context(regs) == 0 {
            // 다음 스레드로 컨텍스트 스위칭
            let next = CONTEXTS.front().unwrap();
            switch_context((**next).get_regs());
        }

        // 불필요한 스택 영역을 삭제
        rm_unused_stack(); // ❺
    }
}

extern "C" fn entry_point() {
    unsafe {
        // 지정된 엔트리 함수를 실행 ❶
        let ctx = CONTEXTS.front().unwrap();
        ((**ctx).entry)();

        // 아래는 스레드 종료 시 후처리

        // 자신의 컨텍스트를 제거
        let ctx = CONTEXTS.pop_front().unwrap();

        // 스레드 ID를 삭제
        (*ID).remove(&ctx.id);

        // 불필요한 스택 영역으로서 보존
        // 이 단계에서 해제하면 다음 코드에서 스택을 사용할 수 없게 됨
        UNUSED_STACK = ((*ctx).stack, (*ctx).stack_layout); // ❷

        match CONTEXTS.front() { // ❸
            Some(c) => {
                // 다음 스레드로 컨텍스트 스위칭
                switch_context((**c).get_regs());
            }
            None => {
                // 모든 스레드가 종료되었다면 main 함수의 스레드로 돌아감
                if let Some(c) = &CTX_MAIN {
                    switch_context(&**c as *const Registers);
                }
            }
        };
    }
    panic!("entry_point"); // ❹
}

pub fn spawn_from_main(func: Entry, stack_size: usize) {
    unsafe {
        // 이미 초기화를 했다면 에러가 된다
        if let Some(_) = &CTX_MAIN {
            panic!("spawn_from_main is called twice");
        }

        // main 함수용 컨텍스트를 생성
        CTX_MAIN = Some(Box::new(Registers::new(0)));
        if let Some(ctx) = &mut CTX_MAIN {
            // 글로벌 변수를 초기화 ❶
            let mut msgs = MappedList::new();
            MESSAGES = &mut msgs as *mut MappedList<u64>;

            let mut waiting = HashMap::new();
            WAITING = &mut waiting as *mut HashMap<u64, Box<Context>>;

            let mut ids = HashSet::new();
            ID = &mut ids as *mut HashSet<u64>;

            // 모든 스레드 종료 시 돌아갈 위치를 저장 ❷
            if set_context(&mut **ctx as *mut Registers) == 0 {
                // 최초에 실행하는 스레드의 컨텍스트를 생성 ❸
                CONTEXTS.push_back(Box::new(Context::new(func, stack_size, get_id())));
                let first = CONTEXTS.front().unwrap();
                switch_context(first.get_regs());
            }

            // 불필요한 스택을 해제 ❹
            rm_unused_stack();

            // 글로벌 변수 클리어
            CTX_MAIN = None;
            CONTEXTS.clear();
            MESSAGES = ptr::null_mut();
            WAITING = ptr::null_mut();
            ID = ptr::null_mut();

            msgs.clear(); // ❺
            waiting.clear();
            ids.clear();
        }
    }
}

unsafe fn rm_unused_stack() {
    if UNUSED_STACK.0 != ptr::null_mut() {
        // 스택 영역 보호 해제 ❶
        mprotect(
            UNUSED_STACK.0 as *mut c_void,
            PAGE_SIZE,
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
        )
        .unwrap();
        // ス스택 영역 해제 ❷
        dealloc(UNUSED_STACK.0, UNUSED_STACK.1);
        UNUSED_STACK = (ptr::null_mut(), Layout::new::<u8>());
    }
}

pub fn send(key: u64, msg: u64) { // ❶
    unsafe {
        // 메시지 큐의 맨 끝에 추가
        (*MESSAGES).push_back(key, msg);

        //스레드 수신 대기 시 실행 큐로 이동
        if let Some(ctx) = (*WAITING).remove(&key) {
            CONTEXTS.push_back(ctx);
        }
    }
    schedule(); // ❷
}

pub fn recv() -> Option<u64> {
    unsafe {
        // 스레드 ID를 취득
        let key = CONTEXTS.front().unwrap().id;

        // 메시지가 이미 큐에 있으면 즉시 리턴
        if let Some(msg) = (*MESSAGES).pop_front(key) {
            return Some(msg);
        }

        // 실행 가능한 스레드가 달리 없으면 데드록
        if CONTEXTS.len() == 1 {
            panic!("deadlock");
        }

        // 실행 중 스레드를 수신 대기 상대로 이동
        let mut ctx = CONTEXTS.pop_front().unwrap();
        let regs = ctx.get_regs_mut();
        (*WAITING).insert(key, ctx);

        // 다음 실행 가능한 스레드로 컨텍스트 스위칭
        if set_context(regs) == 0 {
            let next = CONTEXTS.front().unwrap();
            switch_context((**next).get_regs());
        }

        // 불필요한 스택을 삭제
        rm_unused_stack();

        // 수신한 메시지를 취득
        (*MESSAGES).pop_front(key)
    }
}