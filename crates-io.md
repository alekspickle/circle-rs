# circle-rs

[![version](https://img.shields.io/crates/v/circle-rs)](https:://github.com/alekspickle)

## Minimalistic modern infinite terminal progress indicator

This is slightly changed version of [rustbar](https://crates.io/crates/rustbar) crate, which is simple and minimalistic,
but i needed another infinite bar animation, hence this crate.

#### The goal also was to be able to use it as simple as:

## Example
```rust
#use std::{io::Result, thread, time::Duration};

#use wait_lib::{Infinite, Progress};
pub fn main() -> Result<()> {
    println!("\nGoing to poll some stuff.");
    let mut loader = Infinite::new().to_stderr();
    loader.set_msg("Polling");
    loader.start()?;
    thread::sleep(Duration::from_secs(2));
    loader.stop()?;
    Ok(())
}
```
## Features:
1. set custom loading message
2. set loader speed without reconstructing it


License: MIT
