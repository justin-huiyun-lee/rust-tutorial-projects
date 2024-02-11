use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut seconds = Duration::from_secs(900);
    let mut alternator: bool = true;

    let _start = Instant::now();
    loop {
        thread::sleep(seconds.clone());

        if alternator {
            println!("Time to Stand Up!");
            seconds = Duration::from_secs(15);
        } else {
            println!("Time to Get Back to Work!");
            seconds = Duration::from_secs(900);
        }

        alternator = !alternator;
    }
}
