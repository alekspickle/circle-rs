//! [![version](https://img.shields.io/crates/v/circle-rs)](https:://github.com/alekspickle)
//!
//! # Minimalistic modern infinite terminal progress indicator
//!
//! This is slightly changed version of [rustbar](https://crates.io/crates/rustbar) crate, which is simple and minimalistic,
//! but i needed another infinite bar animation, hence this crate.
//!
//! ### The goal also was to be able to use it as simple as:
//!
//! # Example
//! ```rust,no_run
//! use std::{io::Result, thread, time::{Duration, Instant}};
//! use circle_rs::{Infinite, Progress};
//!
//! pub fn main() -> Result<()> {
//!     println!("\n100 ms delay");
//!     let mut loader = Infinite::new().to_stderr();
//!     loader.set_msg("Polling");
//!     let start_thread = loader.start()?;
//!     let now = Instant::now();
//!     thread::sleep(Duration::from_secs(2));
//!     loader.stop()?;
//!     println!("elapsed {} {:?}",start_thread, now.elapsed());
//!     Ok(())
//! }
//! ```
//! # Features:
//! 1. set custom loading message
//! 2. set loader speed without reconstructing it
//! 3. add cute greeny "done" message after loader is done
//! 
//! ## Note: 
//!  In (3) case you'll need to enable feature, because it requires termion to be added.
//!  Because you **dont pay for what you dont want**, right?
//! ```toml
//! [dependencies]
//! circle-rs = {version = "*", features = ["end"]}
//! ```
//!
use std::{
    io::{stderr, stdout, Result, Write},
    thread,
    time::Duration,
};

#[cfg(feature = "end")]
mod utils;

/// loader pattern
const PATTERN: &str = "⠁⠁⠂⠂⠄⠄⡀⡀⡀⠠⠠⠐⠐⠈";

fn clear_stdout() -> Result<()> {
    stdout().flush()?;
    Ok(())
}

fn write_to_stdout(buf: String) -> Result<()> {
    let mut output = stdout();
    output.write(buf.as_bytes())?;
    output.flush()?;
    Ok(())
}

fn write_to_stderr(buf: String) -> Result<()> {
    let mut output = stderr();
    output.write(buf.as_bytes())?;
    output.flush()?;
    Ok(())
}

/// Main trait
pub trait Progress<T> {
    fn new() -> T;
    fn to_stderr(&mut self) -> T;
    fn write(&self, buf: String) -> Result<()>;
    #[cfg(feature = "end")]
    fn write_done(&self) -> Result<()>;
    fn clear(&self) -> Result<()>;
}

/// Struct for storing state
#[derive(Clone)]
pub struct Infinite {
    msg: String,
    marker_position: u8,
    step: u8,
    delay: Duration,
    write_fn: fn(String) -> Result<()>,
    clear_fn: fn() -> Result<()>,
    rolling: bool,
    done: bool,
}

impl Default for Infinite {
    fn default() -> Infinite {
        Infinite {
            step: 1,
            delay: Duration::from_millis(100),
            msg: "".to_owned(),
            marker_position: 0,
            write_fn: write_to_stdout,
            clear_fn: clear_stdout,
            rolling: false,
            done: false,
        }
    }
}

impl Progress<Infinite> for Infinite {
    fn new() -> Infinite {
        Infinite {
            ..Default::default()
        }
    }

    fn to_stderr(&mut self) -> Infinite {
        self.write_fn = write_to_stderr;
        self.clone()
    }

    fn write(&self, buf: String) -> Result<()> {
        (self.write_fn)(buf)
    }
    #[cfg(feature = "end")]
    fn write_done(&self) -> Result<()> {
        let buf = utils::print_green("done\n");
        (self.write_fn)(buf)
    }
    fn clear(&self) -> Result<()> {
        (self.clear_fn)()?;
        Ok(())
    }
}

impl Infinite {
    pub fn set_msg(&mut self, msg: &str) {
        self.msg = msg.to_owned()
    }
    pub fn set_delay(&mut self, d: Duration) {
        self.delay = d
    }
    pub fn set_done(&mut self, done: bool) {
        self.done = done
    }

    pub fn get_msg(&self) -> &str {
        self.msg.as_ref()
    }
    pub fn get_delay(&self) -> Duration {
        self.delay
    }

    pub fn stop(&mut self) -> Result<()> {
        self.rolling = false;
        self.clear()?;
        self.render_end()?;
        Ok(())
    }
    pub fn start<'a>(&'a mut self) -> Result<String> {
        self.rolling = true;
        let mut bar = self.clone();
        let sleep = self.delay;
        let thread_name = "rolling";
        thread::Builder::new()
            .name(thread_name.clone().into())
            .spawn(move || loop {
                bar.render().unwrap();
                thread::sleep(sleep);
            })?;
        Ok(thread_name.into())
    }

    #[cfg(feature = "end")]
    pub fn render_end(&mut self) -> Result<()> {
        if self.done {
            self.write_done()
        } else {
            self.write("\r".into())
        }
    }
    #[cfg(not(feature = "end"))]
    pub fn render_end(&mut self) -> Result<()> {
        self.write("\r".into())
    }
    pub fn render(&mut self) -> Result<()> {
        if self.marker_position == 0 {
            self.marker_position = 0;
            self.step = 1;
        } else if self.marker_position == 12 {
            self.marker_position = 0;
            self.step = 1;
        }
        self.marker_position = self.marker_position + self.step;

        let bar = PATTERN
            .chars()
            .nth(usize::from(self.marker_position))
            .expect("out of bounds");

        self.write(format!(
            "\r{msg} {bar}{bar}{bar} ",
            msg = self.msg,
            bar = bar
        ))?;
        Ok(())
    }
}
