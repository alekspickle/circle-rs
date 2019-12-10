use std::{thread, time::Duration};
use wait_lib::{InfiniteProgressBar, ProgressBar};

pub fn main() {
    println!("\nInifiniteProgressBar demo");
    let mut infbar = InfiniteProgressBar::new().to_stderr();
    for _ in 0.. {
        infbar.set_msg("Thinking...");
        infbar.render().unwrap();
        thread::sleep(Duration::from_secs(20));
    }
}
