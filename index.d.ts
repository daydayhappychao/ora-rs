/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface Spinner {
  interval: number
  frames: Array<string>
}
export interface Options {
  /**
  *  Text to display after the spinner.
  */
  text?: string
  /**
  Text or a function that returns text to display before the spinner. No prefix text will be displayed if set to an empty string.
  */
  prefixText?: string
  prefixTextFn?: string
  /**
  Text or a function that returns text to display after the spinner text. No suffix text will be displayed if set to an empty string.
  */
  suffixText?: string
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
  spinner?: Spinner
  /**
  The color of the spinner.
  
  @default 'cyan'
  */
  color?: string
  /**
  Set to `false` to stop Ora from hiding the cursor.
  
  @default true
  */
  hideCursour?: boolean
  /**
  Indent the spinner with the given number of spaces.
  
  @default 0
  */
  indent?: number
  /**
  Interval between each frame.
  
  Spinners provide their own recommended interval, so you don't really need to specify this.
  
  Default: Provided by the spinner or `100`.
  */
  interval?: number
  /**
  Force enable/disable the spinner. If not specified, the spinner will be enabled if the `stream` is being run inside a TTY context (not spawned or piped) and/or not in a CI environment.
  
  Note that `{isEnabled: false}` doesn't mean it won't output anything. It just means it won't output the spinner, colors, and other ansi escape codes. It will still log text.
  */
  isEnable?: boolean
  /**
  Disable the spinner and all log text. All output is suppressed and `isEnabled` will be considered `false`.
  
  @default false
  */
  isSilent?: boolean
  /**
  Discard stdin input (except Ctrl+C) while running if it's TTY. This prevents the spinner from twitching on input, outputting broken lines on `Enter` key presses, and prevents buffering of input while the spinner is running.
  
  This has no effect on Windows as there's no good way to implement discarding stdin properly there.
  
  @default true
  */
  discardStdin?: boolean
}
export class Ora {
  lineCount: number
  indent: number
  isSlient: boolean
  isEnable: boolean
  isTty: boolean
  constructor(options: Options)
  start(text?: string | undefined | null): void
}
