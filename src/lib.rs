mod option;

use std::{
  sync::{Arc, Mutex},
  thread::{self, JoinHandle},
  time::Duration,
  usize,
};

use console::Term;
use napi_derive::napi;
use option::Options;
use termion::terminal_size;
use unicode_width::UnicodeWidthStr;

type SharedData<T> = Arc<Mutex<T>>;

#[napi(js_name = "OraBinding")]
pub struct Ora {
  log_without_tty: SharedData<bool>,
  enable: SharedData<bool>,
  text: Arc<Mutex<String>>,
  interval: Arc<Mutex<usize>>,
  frames: Arc<Mutex<Vec<&'static str>>>,
  frame_index: Arc<Mutex<usize>>,
  line_count: Arc<Mutex<usize>>,
  stop_flag: Arc<Mutex<bool>>,
  join: Option<JoinHandle<()>>,
  stream: Arc<Mutex<Term>>,
  first_render: Arc<Mutex<bool>>,
  hide_cursor: Arc<Mutex<bool>>,
  prefix_text: Arc<Mutex<String>>,
  suffix_text: Arc<Mutex<String>>,
  pub is_spinning: bool,
  pub is_tty: bool,
}

#[napi]
impl Ora {
  #[napi(constructor)]
  pub fn new(options: Options) -> Self {
    let stream = Term::stderr();
    let ora = Self {
      log_without_tty: Arc::new(Mutex::new(options.log_without_tty)),
      enable: Arc::new(Mutex::new(options.enable)),
      text: Arc::new(Mutex::new(
        options.text.unwrap_or("".to_string()).to_string(),
      )),
      interval: Arc::new(Mutex::new(options.interval as usize)),
      frames: Arc::new(Mutex::new(options.frames)),
      frame_index: Arc::new(Mutex::new(0)),
      line_count: Arc::new(Mutex::new(1)),
      is_spinning: false,
      join: None,
      stream: Arc::new(Mutex::new(stream.clone())),
      stop_flag: Arc::new(Mutex::new(false)),
      is_tty: termion::is_tty(&stream),
      first_render: Arc::new(Mutex::new(true)),
      hide_cursor: Arc::new(Mutex::new(options.hide_cursor)),
      prefix_text: Arc::new(Mutex::new(options.prefix_text.unwrap_or("".to_string()))),
      suffix_text: Arc::new(Mutex::new(options.suffix_text.unwrap_or("".to_string()))),
    };

    ora.sync_line_count();
    ora
  }

  #[napi]
  pub fn run(&mut self) -> () {
    let self_text = Arc::clone(&self.text);
    let interval = Arc::clone(&self.interval);
    let frames = Arc::clone(&self.frames);
    let stream = Arc::clone(&self.stream);
    let frame_index = Arc::clone(&self.frame_index);
    let stop_flag = Arc::clone(&self.stop_flag);
    let first_render = Arc::clone(&self.first_render);
    let line_count = Arc::clone(&self.line_count);
    let hide_cursor = Arc::clone(&self.hide_cursor);
    let prefix_text = Arc::clone(&self.prefix_text);
    let suffix_text = Arc::clone(&self.suffix_text);

    self.is_spinning = true;
    self.set_stop_flag(false);

    {
      let mut inner_first_render = first_render.lock().unwrap();
      *inner_first_render = true;
    }

    let join = thread::spawn(move || loop {
      let copy_inner_interval;
      {
        let inner_stop = stop_flag.lock().unwrap();
        if *inner_stop {
          break;
        }

        let inner_stream = stream.lock().unwrap();
        let mut inner_frame_index = frame_index.lock().unwrap();
        let inner_text = self_text.lock().unwrap();
        let inner_frames = frames.lock().unwrap();
        let mut inner_first_render = first_render.lock().unwrap();
        let inner_line_count = line_count.lock().unwrap();
        let inner_prefix_text = prefix_text.lock().unwrap();
        let inner_suffix_text = suffix_text.lock().unwrap();

        if *inner_frame_index > inner_frames.len() - 1 {
          *inner_frame_index = 0;
        }

        let frame = inner_frames.get(*inner_frame_index as usize).unwrap();
        let final_prefix_text = if inner_prefix_text.eq("") {
          "".to_string()
        } else {
          (*inner_prefix_text).clone() + " "
        };
        let final_text = if inner_text.eq("") {
          "".to_string()
        } else {
          " ".to_string() + &inner_text
        };
        let final_suffix_text = if inner_suffix_text.eq("") {
          "".to_string()
        } else {
          (*inner_suffix_text).clone() + " "
        };
        let text = format!(
          "{}{}{}{}",
          final_prefix_text, frame, final_text, final_suffix_text
        );

        if *inner_first_render == true {
          *inner_first_render = false;
          let inner_hide_cursor = hide_cursor.lock().unwrap();
          if *inner_hide_cursor {
            inner_stream.hide_cursor().unwrap();
          }
        } else {
          inner_stream.clear_last_lines(*inner_line_count).unwrap();
        }
        inner_stream.write_line(text.as_str()).unwrap();

        *inner_frame_index += 1;

        let innter_interval = interval.lock().unwrap();
        copy_inner_interval = *innter_interval as u64;
      }

      thread::sleep(Duration::from_millis(copy_inner_interval));
    });

    self.join = Some(join);
  }

  #[napi]
  pub fn stop(&mut self) -> () {
    self.set_stop_flag(true);
    let bind_stream = Arc::clone(&self.stream);
    let bind_line_count = Arc::clone(&self.line_count);
    let bind_hide_cursor = Arc::clone(&self.hide_cursor);
    let inner_stream = bind_stream.lock().unwrap();
    let inner_line_count = bind_line_count.lock().unwrap();
    let hide_cursor = bind_hide_cursor.lock().unwrap();

    inner_stream.clear_last_lines(*inner_line_count).unwrap();
    if *hide_cursor {
      inner_stream.show_cursor().unwrap();
    }
  }

  #[napi]
  pub fn pure_write(&self, text: String) -> () {
    let stream = Arc::clone(&self.stream);
    let inner_stream = stream.lock().unwrap();
    let _ = inner_stream.write_line(text.as_str());
  }

  #[napi]
  pub fn clear(&self) -> () {}

  pub fn course_hide(&self) {
    let bind_stream = Arc::clone(&self.stream);
    let inner_stream = bind_stream.lock().unwrap();
    inner_stream.hide_cursor().unwrap();
  }

  pub fn course_show(&self) {
    let bind_stream = Arc::clone(&self.stream);
    let inner_stream = bind_stream.lock().unwrap();
    inner_stream.show_cursor().unwrap();
  }

  fn set_stop_flag(&mut self, next_stop_flag: bool) -> () {
    let stop_flag = Arc::clone(&self.stop_flag);
    if next_stop_flag == true {
      self.is_spinning = false;
    } else {
      self.is_spinning = true;
    }
    let mut inner_stop_flag = stop_flag.lock().unwrap();
    *inner_stop_flag = next_stop_flag;
  }

  fn sync_line_count(&self) {
    let binding_line_count = Arc::clone(&self.line_count).clone();
    let binding_text = Arc::clone(&self.text).clone();
    let inner_text = binding_text.lock().unwrap();
    let mut inner_line_count = binding_line_count.lock().unwrap();
    let (columns, rows) = terminal_size().unwrap_or((80, 80));
    let pure_text = console::strip_ansi_codes(inner_text.as_str());
    // 1 is spinner frame，other 1 is whitespace
    let pure_text_size = pure_text.width() + 1 + 1;
    let line_count = pure_text_size.div_ceil(columns.into()) as usize;
    *inner_line_count = line_count;
  }

  #[napi]
  pub fn update_text(&self, text: String) -> () {
    let binding_text = Arc::clone(&self.text).clone();
    let mut inner_text = binding_text.lock().unwrap();
    *inner_text = text.clone();
    self.sync_line_count();
  }

  #[napi]
  pub fn update_interval(&self, interval: u32) -> () {
    let binding = Arc::clone(&self.interval).clone();
    let mut inner_interval = binding.lock().unwrap();
    *inner_interval = interval as usize;
  }

  #[napi]
  pub fn update_frames(&self, frames: Vec<&'static str>) -> () {
    let binding_frames = Arc::clone(&self.frames).clone();
    let mut inner_frames = binding_frames.lock().unwrap();
    *inner_frames = frames;
    let binding_frame_index = Arc::clone(&self.frame_index).clone();
    let mut inner_frame_index = binding_frame_index.lock().unwrap();
    *inner_frame_index = 0;
  }
}
