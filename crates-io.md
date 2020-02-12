# circle-rs

[![version](https://img.shields.io/crates/v/circle-rs)](https:://github.com/alekspickle)

## Minimalistic modern infinite terminal progress indicator

This is slightly changed version of [rustbar](https://crates.io/crates/rustbar) crate, which is simple and minimalistic,
but i needed another infinite bar animation, hence this crate.

#### The goal also was to be able to use it as simple as:

## Example
```rust
use std::{io::Result, thread, time::{Duration, Instant}};
use circle_rs::{Infinite, Progress};

pub fn main() -> Result<()> {
    println!("\n100 ms delay");
    let mut loader = Infinite::new().to_stderr();
    loader.set_msg("Polling");
    let start_thread = loader.start()?;
    let now = Instant::now();
    thread::sleep(Duration::from_secs(2));
    loader.stop()?;
    println!("elapsed {} {:?}",start_thread, now.elapsed());
    Ok(())
}
```
## Features:
1. set custom loading message
2. set loader speed without reconstructing it
3. add cute greeny "done" message after loader is done

### Note:
 In (3) case you'll need to enable feature, because it requires termion to be added.
 Because you **dont pay for what you dont want**, right?
```toml
[dependencies]
circle-rs = {version = "*", features = ["end"]}
```


License: MIT OR Apache-2.0
