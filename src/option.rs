use napi_derive::napi;

#[napi(object)]
#[derive(Debug, Clone)]
pub struct Spinner {
  pub interval: i32,
  pub frames: Vec<&'static str>,
}

type Frames = Vec<&'static str>;

#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct Options {
  pub text: Option<String>,

  pub interval: i32,

  #[napi(ts_type = "string[]")]
  pub frames: Frames,

  pub log_without_tty: bool,

  pub enable: bool,
}
