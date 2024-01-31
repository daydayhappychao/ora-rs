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

use console::Term;
use napi_derive::napi;
use option::{Options, Spinner};
use strip_ansi_escapes::strip_str;
use termion::terminal_size;

type ChannelData = (Option<i32>, Option<Vec<&'static str>>);

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
  pub is_spinning: bool,
  pub is_tty: bool,
}

#[napi]
impl Ora {
  #[napi(constructor)]
  pub fn new(options: Options) -> Self {
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
      stream: Arc::new(Mutex::new(Term::stderr())),
      stop_flag: Arc::new(Mutex::new(false)),
      is_tty: atty::is(atty::Stream::Stderr),
      first_render: Arc::new(Mutex::new(true)),
    };

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

        if *inner_frame_index > inner_frames.len() - 1 {
          *inner_frame_index = 0;
        }

        let frame = inner_frames.get(*inner_frame_index as usize).unwrap();
        let text = (*inner_text).as_str();

        if *inner_first_render == true {
          *inner_first_render = false;
        } else {
          let _ = inner_stream.clear_last_lines(*inner_line_count);
        }
        let _ = inner_stream.write_line(format!("{}{}", frame, text).as_str());

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
    self.set_stop_flag(true)
  }

  #[napi]
  pub fn pure_write(&self, text: String) -> () {
    let stream = Arc::clone(&self.stream);
    let inner_stream = stream.lock().unwrap();
    let _ = inner_stream.write_line(text.as_str());
  }

  #[napi]
  pub fn update_text(&self, text: String) -> () {
    let binding = Arc::clone(&self.text).clone();
    let mut inner_text = binding.lock().unwrap();
    *inner_text = text;
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

  #[napi]
  pub fn clear(&self) -> () {}

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

  pub fn render(&self) -> &Ora {
    &self.clear();

    self
  }

  pub fn frame(&self) -> &str {
    "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"
  }
}
