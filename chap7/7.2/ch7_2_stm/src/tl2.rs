use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::atomic::{fence, AtomicU64, Ordering};

// 스트라이프 크기
const STRIPE_SIZE: usize = 8; // u64, 8바이트

// 메모리 합계 크기
const MEM_SIZE: usize = 512; // 512바이트

// 메모리 타입
pub struct Memory {
    mem: Vec<u8>,             // 메모리
    lock_ver: Vec<AtomicU64>, // lock & version
    global_clock: AtomicU64,  // global version-clock

    // 주소에서 스트라이프 번호로 변환하기 위한 시프트(이동)량
    shift_size: u32,
}

impl Memory {
    pub fn new() -> Self { // ❶
        // 메모리 영역을 생성
        let mem = [0].repeat(MEM_SIZE);

        // 주소에서 스트라이프 번호로 변환하기 위한 시프트량을 계산
        // 스트라이프의 크기는 2^n에 얼라인먼트되어야 함
        let shift = STRIPE_SIZE.trailing_zeros(); // ❷

        // lock & version을 초기화 ❸
        let mut lock_ver = Vec::new();
        for _ in 0..MEM_SIZE >> shift {
            lock_ver.push(AtomicU64::new(0));
        }

        Memory {
            mem,
            lock_ver,
            global_clock: AtomicU64::new(0),
            shift_size: shift,
        }
    }

    // global version-clock을 인크리먼트 ❹
    fn inc_global_clock(&mut self) -> u64 {
        self.global_clock.fetch_add(1, Ordering::AcqRel)
    }

    // 대상 주소의 버전을 취득 ❺
    fn get_addr_ver(&self, addr: usize) -> u64 {
        let idx = addr >> self.shift_size;
        let n = self.lock_ver[idx].load(Ordering::Relaxed);
        n & !(1 << 63)
    }

    // 대상 주소의 버전아 rv 이하로 록 되어있지 않은지 확인 ❻
    fn test_not_modify(&self, addr: usize, rv: u64) -> bool {
        let idx = addr >> self.shift_size;
        let n = self.lock_ver[idx].load(Ordering::Relaxed);
        // 록의 비트는 최상위 비트로 하므로,
        // rv와 비교하는 것만으로 간단히 확인 가능
        n <= rv
    }

    // 대상 주소의 록을 획득 ❼
    fn lock_addr(&mut self, addr: usize) -> bool {
        let idx = addr >> self.shift_size;
        match self.lock_ver[idx].fetch_update( // ❽
            Ordering::Relaxed, // 쓰기 시의 오더
            Ordering::Relaxed, // 일기 시의 오더
            |val| {
                // 최상위 비트 값 확인 및 설정
                let n = val & (1 << 63);
                if n == 0 {
                    Some(val | (1 << 63))
                } else {
                    None
                }
            },
        ) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    // 대상 주소의 록 해제 ❾
    fn unlock_addr(&mut self, addr: usize) {
        let idx = addr >> self.shift_size;
        self.lock_ver[idx].fetch_and(!(1 << 63),
                                     Ordering::Relaxed);
    }
}

pub struct ReadTrans<'a> { // ❶
    read_ver: u64,   // read-version
    is_abort: bool,  // 경쟁을 감지하면 True
    mem: &'a Memory, // Memory 타입으로의 참조
}

impl<'a> ReadTrans<'a> {
    fn new(mem: &'a Memory) -> Self { // ❷
        ReadTrans {
            is_abort: false,

            // global version-clock 읽기
            read_ver: mem.global_clock.load(Ordering::Acquire),

            mem,
        }
    }

    // 메모리 읽기 함수 ❸
    pub fn load(&mut self, addr: usize) -> Option<[u8; STRIPE_SIZE]> {
        // 경쟁을 감지하면 종료 ❹
        if self.is_abort {
            return None;
        }

        // 주소가 스트라이프의 얼라인먼트에 맞는가를 확인
        assert_eq!(addr & (STRIPE_SIZE - 1), 0); // ❺

        // 읽기 메모리가 록 되어있지 않고, read-version 이하인가 확인 ❻
        if !self.mem.test_not_modify(addr, self.read_ver) {
            self.is_abort = true;
            return None;
        }

        fence(Ordering::Acquire);

        // 메모리 읽기. 단순한 복사임 ❼
        let mut mem = [0; STRIPE_SIZE];
        for (dst, src) in mem
            .iter_mut()
            .zip(self.mem.mem[addr..addr + STRIPE_SIZE].iter())
        {
            *dst = *src;
        }

        fence(Ordering::SeqCst);

        // 읽기 메모리가 록 되어있지 않고, read-version 이하인가 확인 ❽
        if !self.mem.test_not_modify(addr, self.read_ver) {
            self.is_abort = true;
            return None;
        }

        Some(mem)
    }
}

pub struct WriteTrans<'a> {
    read_ver: u64,            // read-version
    read_set: HashSet<usize>, // read-set
    write_set: HashMap<usize, [u8; STRIPE_SIZE]>, // write-set
    locked: Vec<usize>,  // 록 완료 주소
    is_abort: bool,      // 경쟁을 감지하면 true
    mem: &'a mut Memory, // Memory 타입으로의 참조
}

impl<'a> Drop for WriteTrans<'a> {
    fn drop(&mut self) {
        // 록 완료 주소의 록을 해제
        for addr in self.locked.iter() {
            self.mem.unlock_addr(*addr);
        }
    }
}

impl<'a> WriteTrans<'a> {
    fn new(mem: &'a mut Memory) -> Self { // ❶
        WriteTrans {
            read_set: HashSet::new(),
            write_set: HashMap::new(),
            locked: Vec::new(),
            is_abort: false,

            // global version-clock 읽기
            read_ver: mem.global_clock.load(Ordering::Acquire),

            mem,
        }
    }

    // 메모리 쓰기 함수 ❷
    pub fn store(&mut self, addr: usize, val: [u8; STRIPE_SIZE]) {
        // 주소가 스트라이브의 얼라인먼트에 맞는가 확인
        assert_eq!(addr & (STRIPE_SIZE - 1), 0);
        self.write_set.insert(addr, val);
    }

    // 메모리 읽기 함수 ❸
    pub fn load(&mut self, addr: usize) -> Option<[u8; STRIPE_SIZE]> {
        // 경쟁을 감지한 경우 종료
        if self.is_abort {
            return None;
        }

        // 주소가 스트라이프의 얼라인먼트에 맞는가 확인
        assert_eq!(addr & (STRIPE_SIZE - 1), 0);

        // 읽기 주소 저장
        self.read_set.insert(addr);

        // write-set에 있다면 이를 읽음
        if let Some(m) = self.write_set.get(&addr) {
            return Some(*m);
        }

        // 읽기 메모리가 록 되어있지 않고, read-version 이하인지 판정
        if !self.mem.test_not_modify(addr, self.read_ver) {
            self.is_abort = true;
            return None;
        }

        fence(Ordering::Acquire);

        // 메모리 읽기. 단순히 복사함.
        let mut mem = [0; STRIPE_SIZE];
        for (dst, src) in mem
            .iter_mut()
            .zip(self.mem.mem[addr..addr + STRIPE_SIZE].iter())
        {
            *dst = *src;
        }

        fence(Ordering::SeqCst);

        // 읽기 메모리가 록 되어있지 않고, read-version 이하인지 판정
        if !self.mem.test_not_modify(addr, self.read_ver) {
            self.is_abort = true;
            return None;
        }

        Some(mem)
    }

    // write-set 안의 주소를 록
    // 모든 주소의 록읠 획득할 수 있는 경우 true를 리턴한다 ❹
    fn lock_write_set(&mut self) -> bool {
        for (addr, _) in self.write_set.iter() {
            if self.mem.lock_addr(*addr) {
                // 록 획득 가능한 경우에는 locked에 추가
                self.locked.push(*addr);
            } else {
                // 가능하지 않은 경우에는 false를 반환하고 종료
                return false;
            }
        }
        true
    }

    // read-set 검증 ❺
    fn validate_read_set(&self) -> bool {
        for addr in self.read_set.iter() {
            // write-set 안에 있는 주소인 경우에는
            // 자기 스레드가 록을 획득한 상태임
            if self.write_set.contains_key(addr) {
                // 버전만 검사
                let ver = self.mem.get_addr_ver(*addr);
                if ver > self.read_ver {
                    return false;
                }
            } else {
                // 다른 스레드가 록을 하지 않았는지와 버전을 검사
                if !self.mem.test_not_modify(*addr, self.read_ver) {
                    return false;
                }
            }
        }
        true
    }

    // 커밋 ❻
    fn commit(&mut self, ver: u64) {
        // 모든 주소에 대해 쓰기. 단순한 메모리 복사임.
        for (addr, val) in self.write_set.iter() {
            let addr = *addr as usize;
            for (dst, src) in self.mem.mem[addr..addr + STRIPE_SIZE].iter_mut().zip(val) {
                *dst = *src;
            }
        }

        fence(Ordering::Release);

        // 모든 주소의 록 해제 및 버전 업데이트
        for (addr, _) in self.write_set.iter() {
            let idx = addr >> self.mem.shift_size;
            self.mem.lock_ver[idx].store(ver, Ordering::Relaxed);
        }

        // 록 완료 주소 집합을 초기화
        self.locked.clear();
    }
}

pub enum STMResult<T> {
    Ok(T),
    Retry, // 트랜잭션 재시도
    Abort, // 트랜잭션 중단
}

pub struct STM {
    mem: UnsafeCell<Memory>, // 실제 메모리
}

// 스레드 사이에서 공유 가능하도록 설정. 채널에서 송수신 가능하도록 설정.
unsafe impl Sync for STM {}
unsafe impl Send for STM {}

impl STM {
    pub fn new() -> Self {
        STM {
            mem: UnsafeCell::new(Memory::new()),
        }
    }

    // 읽기 트랜잭션 ❶
    pub fn read_transaction<F, R>(&self, f: F) -> Option<R>
    where
        F: Fn(&mut ReadTrans) -> STMResult<R>,
    {
        loop {
            // 1. global version-clock 읽기 ❷
            let mut tr = ReadTrans::new(unsafe { &*self.mem.get() });

            // 2. 투기적 실행 ❸
            match f(&mut tr) {
                STMResult::Abort => return None, // 중단
                STMResult::Retry => {
                    if tr.is_abort {
                        continue; // 재시도
                    }
                    return None; // 중단
                }
                STMResult::Ok(val) => {
                    if tr.is_abort == true {
                        continue; // 재시도
                    } else {
                        return Some(val); // 3. 커넷
                    }
                }
            }
        }
    }

    // 쓰기 트랜잭션 ❹
    pub fn write_transaction<F, R>(&self, f: F) -> Option<R>
    where
        F: Fn(&mut WriteTrans) -> STMResult<R>,
    {
        loop {
            // 1. global version-clock 읽기 ❺
            let mut tr = WriteTrans::new(unsafe { &mut *self.mem.get() });

            // 2. 투기적 실행 ❻
            let result;
            match f(&mut tr) {
                STMResult::Abort => return None,
                STMResult::Retry => {
                    if tr.is_abort {
                        continue;
                    }
                    return None;
                }
                STMResult::Ok(val) => {
                    if tr.is_abort {
                        continue;
                    }
                    result = val;
                }
            }

            // 3. write-set 록 ❼
            if !tr.lock_write_set() {
                continue;
            }

            // 4. global version-clock 인크리먼트 ❽
            let ver = 1 + tr.mem.inc_global_clock();

            // 5. read-set 검증 ❾
            if tr.read_ver + 1 != ver && !tr.validate_read_set() {
                continue;
            }

            // 6. 커밋과 릴리스 ❿
            tr.commit(ver);

            return Some(result);
        }
    }
}

// メモリ読み込み用のマクロ ❶
#[macro_export]
macro_rules! load {
    ($t:ident, $a:expr) => {
        if let Some(v) = ($t).load($a) {
            v
        } else {
            // 読み込みに失敗したらリトライ
            return tl2::STMResult::Retry;
        }
    };
}

// メモリ書き込み用のマクロ ❷
#[macro_export]
macro_rules! store {
    ($t:ident, $a:expr, $v:expr) => {
        $t.store($a, $v)
    };
}