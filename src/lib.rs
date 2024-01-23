mod option;
mod spinner;
mod utils;

use std::{
  cmp,
  sync::{
    atomic::AtomicU32,
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
  },
  thread::{self, JoinHandle},
  time::Duration,
};

use napi_derive::napi;
use option::{Options, Spinner};
use strip_ansi_escapes::strip_str;
use termion::terminal_size;
use utils::stream::{self, Stream};

type ChannelData = (Option<i32>, Option<Vec<&'static str>>);

#[napi(js_name = "Ora")]
pub struct Ora {
  options: Options,
  pub line_count: u32,
  pub indent: u32,
  pub is_slient: bool,
  pub is_enable: bool,
  pub is_tty: bool,
  text: Arc<Mutex<String>>,
  is_spinning: Arc<Mutex<bool>>,
  spinner: Arc<Mutex<option::Spinner>>,
  join: Option<JoinHandle<()>>,
  stream: Arc<Mutex<Stream>>,
  frame_index: Arc<Mutex<u32>>,
}

#[napi]
impl Ora {
  #[napi(constructor)]
  pub fn new(options: Options) -> Self {
    let inner_options = Options {
      color: Some("cyan".to_string()),
      hide_cursour: Some(true),
      ..options.clone()
    };

    let mut ora = Self {
      options: inner_options.clone(),
      line_count: 0,
      indent: inner_options.indent.unwrap_or(0),
      text: Arc::new(Mutex::new(
        inner_options.text.unwrap_or("".to_string()).to_string(),
      )),
      is_slient: inner_options.is_silent.unwrap_or(false),
      is_enable: inner_options.is_enable.unwrap_or(true),
      is_spinning: Arc::new(Mutex::new(false)),
      is_tty: atty::is(atty::Stream::Stdout),
      spinner: Arc::new(Mutex::new(inner_options.spinner.unwrap_or(Spinner {
        interval: 80,
        frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
      }))),
      join: None,
      stream: Arc::new(Mutex::new(Stream::default())),
      frame_index: Arc::new(Mutex::new(0)),
    };
    // ora.update_line_count();

    ora
  }

  pub fn spinner() {}

  fn update_line_count(&mut self) {
    let (columns, _rows) = terminal_size().unwrap_or((80, 0));
    let full_prefix_text = self.get_full_prefix_text(&self.options.prefix_text, "-");
    let full_suffix_text = self.get_full_suffix_test(&self.options.suffix_text, " ");

    let text_arc = Arc::clone(&self.text);
    let text = text_arc.lock().unwrap();

    let full_text = format!(
      "{}{}--{}--{}",
      " ".repeat(self.indent.try_into().unwrap()),
      full_prefix_text,
      text,
      full_suffix_text
    );
    let mut next_line_count = 0;
    let strip_text = strip_str(full_text);
    let text_arr = strip_text.split("\n");
    for text in text_arr {
      let addon_cout = cmp::max(
        1,
        unicode_width::UnicodeWidthStr::width(text) / columns as usize,
      );

      next_line_count += addon_cout;
    }
    self.line_count = next_line_count as u32;
  }

  fn get_full_prefix_text(&self, prefix_text: &Option<String>, postfix: &str) -> String {
    match prefix_text {
      Some(prefix_text) => {
        return format!("{}{}", prefix_text, postfix);
      }
      None => "".to_string(),
    }
  }

  fn get_full_suffix_test(&self, suffix_text: &Option<String>, prefix: &str) -> String {
    match suffix_text {
      Some(suffix_text) => {
        return format!("{}{}", prefix, suffix_text);
      }
      None => "".to_string(),
    }
  }

  #[napi]
  pub fn start(&mut self, text: Option<&str>) -> Ora {
    let self_text = Arc::clone(&self.text);
    let spinner = Arc::clone(&self.spinner);
    let stream = Arc::clone(&self.stream);
    let is_spinning = Arc::clone(&self.is_spinning);
    let frame_index = Arc::clone(&self.frame_index);

    {
      let mut inner_is_spinning = is_spinning.lock().unwrap();
      *inner_is_spinning = true;
      let mut text_value = self_text.lock().unwrap();
      *text_value = text.unwrap_or((*text_value).as_str()).to_string();
    }

    let join = thread::spawn(move || 'outer: loop {
      let inner_spinner = spinner.lock().unwrap();
      let inner_stream = stream.lock().unwrap();
      let inner_is_spinning = is_spinning.lock().unwrap();
      let mut inner_frame_index = frame_index.lock().unwrap();
      let inner_text = self_text.lock().unwrap();

      if *inner_is_spinning == false {
        break 'outer;
      }

      let interval = inner_spinner.interval.clone();
      let frames = inner_spinner.frames.clone();

      if *inner_frame_index > frames.len() as u32 - 1 {
        *inner_frame_index = 0;
      }

      inner_stream
        .write(
          inner_spinner
            .frames
            .get(*inner_frame_index as usize)
            .unwrap(),
          (*inner_text).as_str(),
          None,
          None,
        )
        .expect("Write Failed");
      *inner_frame_index += 1;
      thread::sleep(Duration::from_millis(interval as u64));
    });

    self.join = Some(join);
    self
  }

  pub fn render(&self) -> &Ora {
    &self.clear();

    self
  }

  pub fn clear(&self) {}

  pub fn frame(&self) -> &str {
    "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"
  }
}
