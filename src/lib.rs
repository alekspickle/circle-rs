/// this is slightly changed version of rustbar crate, which is simple and minimalistic,
/// but i needed another infinite bar animation, hence this crate.
use std::io::{stderr, stdout, Result, Write};

const PATTERN: &str =  "⠁⠂⠄⡀⢀⠠⠐⠈ ";

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
}

#[derive(Clone)]
pub struct InfiniteProgressBar {
    msg: String,
    marker_position: i8,
    step: i8,
    write_fn: fn(String) -> Result<()>,
}

impl Default for InfiniteProgressBar {
    fn default() -> InfiniteProgressBar {
        InfiniteProgressBar {
            step: 1,
            msg: "".to_owned(),
            marker_position: 0,
            write_fn: write_to_stdout,
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
}

impl InfiniteProgressBar {
    pub fn set_msg(&mut self, msg: &str) {
        self.msg = msg.to_owned()
    }
    pub fn get_msg(&self) -> &str {
        self.msg.as_ref()
    }

    pub fn render(&mut self) -> Result<()> {
        // let (screen_w, screen_h) = term_utils::get_winsize().unwrap();

        if self.marker_position <= 0 {
            self.marker_position = 0;
            self.step = 1;
        } else if self.marker_position > 9 {
            self.marker_position = 10;
            self.step = -1;
        }
        self.marker_position = self.marker_position + self.step;

        let mut bar: String = "..........".to_owned(); //10 dots
        bar.insert(self.marker_position as usize, '#');

        self.write(format!("\r{msg} [{bar}]", msg = self.msg, bar = bar))?;
        Ok(())
    }
}
