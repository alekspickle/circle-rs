use std::{io::Result, thread, time::Duration};
use circle_rs::{Infinite, Progress};

pub fn main() -> Result<()> {
    println!("\nGoing to poll some stuff.");
    let mut infbar = Infinite::new().to_stderr();
    infbar.set_msg("Polling");
    infbar.start()?;
    thread::sleep(Duration::from_secs(2));
    infbar.stop()?;

    Ok(())
}
