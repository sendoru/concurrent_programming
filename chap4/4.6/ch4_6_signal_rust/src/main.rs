use signal_hook::{iterator::Signals, SIGUSR1}; // ❶
use std::{error::Error, process, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    // 프로세스 ID를 표시
    println!("pid: {}", process::id());

    let signals = Signals::new(&[SIGUSR1])?; // ❷
    thread::spawn(move || {
        // 시그널 수신
        for sig in signals.forever() { // ❸
            println!("received signal: {:?}", sig);
        }
    });

    // 10초 슬립
    thread::sleep(Duration::from_secs(10));
    Ok(())
}