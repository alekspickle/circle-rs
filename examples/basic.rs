use std::{io::Result, thread, time::{Duration, Instant}};
use circle_rs::{Infinite, Progress};

pub fn main() -> Result<()> {
    println!("\nGoing to poll some stuff.");
    let mut loader = Infinite::new().to_stderr();
    loader.set_msg("Polling");
    loader.start()?;
    let now = Instant::now();
    thread::sleep(Duration::from_secs(2));
    println!("elapsed {:?}", now.elapsed());
    loader.stop()?;

    Ok(())
}

