use std::sync::{Arc, Mutex};

struct Resource<const NRES: usize, const NTH: usize> {
    available: [usize; NRES],         // 이용 가능한 리소스
    allocation: [[usize; NRES]; NTH], // 스레드 i가 확보 중인 리소스
    max: [[usize; NRES]; NTH],        // 스레드 i가 필요로 하는 리소스의 최댓값
}

impl<const NRES: usize, const NTH: usize> Resource<NRES, NTH> {
    fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
        Resource {
            available,
            allocation: [[0; NRES]; NTH],
            max,
        }
    }

    // 현재 상태가 데드록을 발생시키지 않는가 확인
    fn is_safe(&self) -> bool {
        let mut finish = [false; NTH]; // 스레드 i는 리소스 획득과 반환에 성공했는가?
        let mut work = self.available.clone(); // 이용 가능한 리소스의 시뮬레이션값

        loop {
            // 모든 스레드 i와 리소스 j에 대해,
            // finish[i] == false && work[j] >= (self.max[i][j] - self.allocation[i][j])
            // 을 만족하는 스레드를 찾는다.
            let mut found = false;
            let mut num_true = 0;
            for (i, alc) in self.allocation.iter().enumerate() {
                if finish[i] {
                    num_true += 1;
                    continue;
                }

                // need[j] = self.max[i][j] - self.allocation[i][j] 를 계산하고, 
                // 모든 리소스 j에 대해, work[j] >= need[j] 인가를 판정한다.
                let need = self.max[i].iter().zip(alc).map(|(m, a)| m - a);
                let is_avail = work.iter().zip(need).all(|(w, n)| *w >= n);
                if is_avail {
                    // 스레드 i가 리소스 확보 가능
                    found = true;
                    finish[i] = true;
                    for (w, a) in work.iter_mut().zip(alc) {
                        *w += *a // 스레드 i가 현재 확보하고 있는 리소스를 반환
                    }
                    break;
                }
            }

            if num_true == NTH {
                // 모든 스레드가 리소스 확보 가능하면 안전함
                return true;
            }

            if !found {
                // 스레드가 리소스를 확보할 수 없음
                break;
            }
        }

        false
    }

    // id번 째의 스레드가 resource를 하나 얻음
    fn take(&mut self, id: usize, resource: usize) -> bool {
        // 스레드 번호, 리소스 번호 검사
        if id >= NTH || resource >= NRES || self.available[resource] == 0 {
            return false;
        }

        // 리소스 확보를 시험해 본다
        self.allocation[id][resource] += 1;
        self.available[resource] -= 1;

        if self.is_safe() {
            true // 리소스 확보 성공
        } else {
            // 리소스 확보에 실패했으므로 상태 원복
            self.allocation[id][resource] -= 1;
            self.available[resource] += 1;
            false
        }
    }

    // id번 째의 스레드가 resource를 하나 반환
    fn release(&mut self, id: usize, resource: usize) {
        // 스레드 번호, 리소스 번호를 검사
        if id >= NTH || resource >= NRES || self.allocation[id][resource] == 0 {
            return;
        }

        self.allocation[id][resource] -= 1;
        self.available[resource] += 1;
    }
}

#[derive(Clone)]
pub struct Banker<const NRES: usize, const NTH: usize> {
    resource: Arc<Mutex<Resource<NRES, NTH>>>,
}

impl<const NRES: usize, const NTH: usize> Banker<NRES, NTH> {
    pub fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
        Banker {
            resource: Arc::new(Mutex::new(Resource::new(available, max))),
        }
    }

    pub fn take(&self, id: usize, resource: usize) -> bool {
        let mut r = self.resource.lock().unwrap();
        r.take(id, resource)
    }

    pub fn release(&self, id: usize, resource: usize) {
        let mut r = self.resource.lock().unwrap();
        r.release(id, resource)
    }
}