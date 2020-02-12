use std::{io::Result, thread, time::{Duration, Instant}};
use circle_rs::{Infinite, Progress};

pub fn main() -> Result<()> {
    println!("\n100 ms delay");
    let mut loader = Infinite::new().to_stderr();
    loader.set_msg("Polling");
    loader.set_done(true);
    
    let start_thread = loader.start()?;
    let now = Instant::now();
    thread::sleep(Duration::from_secs(2));
    loader.stop()?;
    println!("elapsed {} {:?}",start_thread, now.elapsed());
    
    Ok(())
}

