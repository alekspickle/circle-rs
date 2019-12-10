/// this is slightly changed version of rustbar crate, which is simple and minimalistic,
/// but i needed another infinite bar animation, hence this crate.
use std::{
    io::{stderr, stdout, Result, Write},
    thread,
    time::Duration,
};

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

///all progressbars will implement it
pub trait ProgressBar<T> {
    fn new() -> T;
    fn to_stderr(&mut self) -> T;
    fn write(&self, buf: String) -> Result<()>;
    fn clear(&self) -> Result<()>;
}

#[derive(Clone)]
pub struct InfiniteProgressBar {
    msg: String,
    marker_position: u8,
    step: u8,
    write_fn: fn(String) -> Result<()>,
    clear_fn: fn() -> Result<()>,
    rolling: bool,
}

impl Default for InfiniteProgressBar {
    fn default() -> InfiniteProgressBar {
        InfiniteProgressBar {
            step: 1,
            msg: "".to_owned(),
            marker_position: 0,
            write_fn: write_to_stdout,
            clear_fn: clear_stdout,
            rolling: false,
        }
    }
}

impl ProgressBar<InfiniteProgressBar> for InfiniteProgressBar {
    fn new() -> InfiniteProgressBar {
        InfiniteProgressBar {
            ..Default::default()
        }
    }

    fn to_stderr(&mut self) -> InfiniteProgressBar {
        self.write_fn = write_to_stderr;
        self.clone()
    }

    fn write(&self, buf: String) -> Result<()> {
        (self.write_fn)(buf)?;
        Ok(())
    }
    fn clear(&self) -> Result<()> {
        (self.clear_fn)()?;
        Ok(())
    }
}

impl InfiniteProgressBar {
    pub fn set_msg(&mut self, msg: &str) {
        self.msg = msg.to_owned()
    }
    pub fn get_msg(&self) -> &str {
        self.msg.as_ref()
    }

    pub fn stop(&mut self) -> Result<()> {
        self.rolling = false;
        println!();
        Ok(())
    }
    pub fn start(&mut self) -> Result<String> {
        self.rolling = true;
        let mut bar = self.clone();
        let thread_name = "rolling";
        thread::Builder::new()
            .name(thread_name.clone().into())
            .spawn(move || loop {
                bar.render().unwrap();
                thread::sleep(Duration::from_millis(100));
            })?;

        Ok(thread_name.into())
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
