use napi_derive::napi;

#[napi(object)]
#[derive(Debug, Clone)]
pub struct Spinner {
  pub interval: i32,
  pub frames: Vec<&'static str>,
}

#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct Options {
  /**
   *  Text to display after the spinner.
   */
  pub text: Option<String>,
  /**
  Text or a function that returns text to display before the spinner. No prefix text will be displayed if set to an empty string.
  */
  pub prefix_text: Option<String>,

  pub prefix_text_fn: Option<String>,
  /**
  Text or a function that returns text to display after the spinner text. No suffix text will be displayed if set to an empty string.
  */
  pub suffix_text: Option<String>,
  /**
  Name of one of the provided spinners. See [`example.js`](https://github.com/BendingBender/ora/blob/main/example.js) in this repo if you want to test out different spinners. On Windows, it will always use the line spinner as the Windows command-line doesn't have proper Unicode support.

  @default 'dots'

  Or an object like:

  @example
  ```
  {
    interval: 80, // Optional
    frames: ['-', '+', '-']
  }
  ```
  */
  pub spinner: Option<Spinner>,
  /**
  The color of the spinner.

  @default 'cyan'
  */
  pub color: Option<String>,
  /**
  Set to `false` to stop Ora from hiding the cursor.

  @default true
  */
  pub hide_cursour: Option<bool>,
  /**
  Indent the spinner with the given number of spaces.

  @default 0
  */
  pub indent: Option<u32>,
  /**
  Interval between each frame.

  Spinners provide their own recommended interval, so you don't really need to specify this.

  Default: Provided by the spinner or `100`.
  */
  pub interval: Option<i32>,

  /**
  Force enable/disable the spinner. If not specified, the spinner will be enabled if the `stream` is being run inside a TTY context (not spawned or piped) and/or not in a CI environment.

  Note that `{isEnabled: false}` doesn't mean it won't output anything. It just means it won't output the spinner, colors, and other ansi escape codes. It will still log text.
  */
  pub is_enable: Option<bool>,

  /**
  Disable the spinner and all log text. All output is suppressed and `isEnabled` will be considered `false`.

  @default false
  */
  pub is_silent: Option<bool>,

  /**
  Discard stdin input (except Ctrl+C) while running if it's TTY. This prevents the spinner from twitching on input, outputting broken lines on `Enter` key presses, and prevents buffering of input while the spinner is running.

  This has no effect on Windows as there's no good way to implement discarding stdin properly there.

  @default true
  */
  pub discard_stdin: Option<bool>,
}
