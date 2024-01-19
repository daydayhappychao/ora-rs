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
  thread,
  time::Duration,
};

use napi_derive::napi;
use option::{Options, Spinner};
use strip_ansi_escapes::strip_str;
use termion::terminal_size;

type ChannelData = (Option<i32>, Option<Vec<&'static str>>);

#[napi(js_name = "Ora")]
pub struct Ora {
  options: Options,
  pub line_count: u32,
  pub indent: u32,
  pub text: String,
  pub is_slient: bool,
  pub is_enable: bool,
  pub is_spinning: bool,
  pub is_tty: bool,
  spinner: Arc<Mutex<option::Spinner>>,
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
      text: inner_options.text.unwrap_or("".to_string()),
      is_slient: inner_options.is_silent.unwrap_or(false),
      is_enable: inner_options.is_enable.unwrap_or(true),
      is_spinning: false,
      is_tty: atty::is(atty::Stream::Stdout),
      spinner: Arc::new(Mutex::new(inner_options.spinner.unwrap_or(Spinner {
        interval: 80,
        frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
      }))),
    };
    ora.update_line_count();

    ora
  }

  pub fn spinner() {}

  fn update_line_count(&mut self) {
    let (columns, _rows) = terminal_size().unwrap_or((80, 0));
    let full_prefix_text = self.get_full_prefix_text(&self.options.prefix_text, "-");
    let full_suffix_text = self.get_full_suffix_test(&self.options.suffix_text, " ");

    let full_text = format!(
      "{}{}--{}--{}",
      " ".repeat(self.indent.try_into().unwrap()),
      full_prefix_text,
      self.text,
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
  pub fn start(&mut self, text: Option<&str>) -> &mut Ora {
    self.text = text.unwrap_or(&self.text).to_string();
    self.is_spinning = true;

    let spinner = Arc::clone(&self.spinner);
    thread::spawn(move || loop {
      let mut spinner = spinner.lock().unwrap();
      thread::sleep(Duration::from_millis(spinner.interval as u64));
    });

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
