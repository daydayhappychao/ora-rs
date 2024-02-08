use std::io::IsTerminal;
use std::io::Stdin;

pub struct stdin_discarder {
  stdin_term: Stdin,
  active_count: u16,
  join: Option<std::thread::JoinHandle<()>>,
}

impl stdin_discarder {
  pub fn start(&mut self) {
    self.active_count += 1;

    if self.active_count == 1 {
      self.real_start();
    }
  }

  #[cfg(not(target_os = "window"))]
  fn real_start(&mut self) {
    use std::thread;

    if !self.stdin_term.is_terminal() {
      return ();
    }

    self.join = Some(thread::spawn(|| {
      let stdin = std::io::stdin();
      let mut stdin = stdin.lock();
      let mut buf = [0u8; 1];
      loop {
        // println!("{:?}", buf);
      }
    }));
  }

  #[cfg(target_os = "windows")]
  fn real_start(&mut self) {}
}
