use std::{thread, time::Duration};
use wait_lib::{InfiniteProgressBar, ProgressBar};

pub fn main() {
    println!("\nStarting to poll some stuff.");
    let mut infbar = InfiniteProgressBar::new().to_stderr();
    infbar.set_msg("Polling");
    infbar.start();
    thread::sleep(Duration::from_secs(2));
    let _ = infbar.stop();

}
