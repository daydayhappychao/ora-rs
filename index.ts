import process from 'node:process';
import cliSpinners from 'cli-spinners';
import logSymbols from 'log-symbols';
import isUnicodeSupported from 'is-unicode-supported';
import stdinDiscarder from 'stdin-discarder';
import { Options, PersistOptions, PrefixTextGenerator, PromiseOptions, Spinner } from './type';
import { OraBinding } from './binding';

class Ora {
    #isDiscardingStdin = false;
    #options: Options;
    #spinner!: Spinner;
    #initialInterval;
    #isEnabled;
    #isSilent;
    #indent: number | undefined;
    #text: string | undefined;
    #prefixText: Options['prefixText'];
    #suffixText: Options['suffixText'];

    private readonly binding: OraBinding;

    color;

    constructor(options: Options) {
        if (typeof options === 'string') {
            options = {
                text: options,
            };
        }

        this.#options = {
            color: 'cyan',
            discardStdin: true,
            hideCursor: true,
            ...options,
        };

        // Public
        this.color = this.#options.color;

        // It's important that these use the public setters.
        this.spinner = this.#options.spinner;

        this.#initialInterval = this.#options.interval;
        this.#isEnabled = typeof this.#options.isEnabled === 'boolean' ? this.#options.isEnabled : true;
        this.#isSilent = typeof this.#options.isSilent === 'boolean' ? this.#options.isSilent : false;

        // Set *after* `this.#stream`.
        // It's important that these use the public setters.
        this.text = this.#options.text;
        this.prefixText = this.#options.prefixText;
        this.suffixText = this.#options.suffixText;
        this.indent = this.#options.indent;

        this.binding = new OraBinding({
            text: this.text,
            interval: this.interval,
            frames: this.spinner.frames,
            logWithoutTty: this.isEnabled,
            enable: this.isSilent
        });
    }

    get indent() {
        return this.#indent;
    }

    set indent(indent: number | undefined) {
        indent = indent || 0;
        if (!(indent >= 0 && Number.isInteger(indent))) {
            throw new Error('The `indent` option must be an integer from 0 and up');
        }

        this.#indent = indent;
        // this.#updateLineCount();
    }

    get interval() {
        return this.#initialInterval ?? this.#spinner.interval ?? 100;
    }

    get spinner(): Spinner {
        return this.#spinner;
    }

    set spinner(spinner: Options['spinner']) {
        this.#initialInterval = undefined;

        if (typeof spinner === 'object') {
            if (spinner.frames === undefined) {
                throw new Error('The given spinner must have a `frames` property');
            }

            this.#spinner = spinner;
        } else if (!isUnicodeSupported()) {
            this.#spinner = cliSpinners.line;
        } else if (spinner === undefined) {
            // Set default spinner
            this.#spinner = cliSpinners.dots;
        } else if (spinner !== 'default' && cliSpinners[spinner]) {
            this.#spinner = cliSpinners[spinner];
        } else {
            throw new Error(`There is no built-in spinner named '${spinner}'. See https://github.com/sindresorhus/cli-spinners/blob/main/spinners.json for a full list.`);
        }
        if (this.binding) {
            this.binding.updateFrames(this.#spinner.frames);
            this.binding.updateInterval(this.#spinner.interval || 100);
        }
    }

    get text() {
        return this.#text;
    }

    set text(value: string | undefined) {
        value = value || '';
        this.#text = value;
        if (this.binding) {

            this.binding.updateText(value);
        }
        // this.#updateLineCount();
    }

    get prefixText(): Options['prefixText'] {
        return this.#prefixText;
    }

    set prefixText(value: Options['prefixText']) {
        this.#prefixText = value;
        // this.#updateLineCount();
    }

    get suffixText(): Options['suffixText'] {
        return this.#suffixText;
    }

    set suffixText(value: Options['suffixText']) {
        this.#suffixText = value;
        // this.#updateLineCount();
    }

    get isSpinning() {
        return this.binding.isSpinning;
    }

    #getFullPrefixText(prefixText = this.#prefixText, postfix = ' ') {
        if (typeof prefixText === 'string' && prefixText !== '') {
            return prefixText + postfix;
        }

        if (typeof prefixText === 'function') {
            return prefixText() + postfix;
        }

        return '';
    }

    #getFullSuffixText(suffixText = this.#suffixText, prefix = ' ') {
        if (typeof suffixText === 'string' && suffixText !== '') {
            return prefix + suffixText;
        }

        if (typeof suffixText === 'function') {
            return prefix + suffixText();
        }

        return '';
    }

    // #updateLineCount() {
    //     const columns = this.#stream.columns ?? 80;
    //     const fullPrefixText = this.#getFullPrefixText(this.#prefixText, '-');
    //     const fullSuffixText = this.#getFullSuffixText(this.#suffixText, '-');
    //     const fullText = ' '.repeat(this.#indent) + fullPrefixText + '--' + this.#text + '--' + fullSuffixText;

    //     this.#lineCount = 0;
    //     for (const line of stripAnsi(fullText).split('\n')) {
    //         this.#lineCount += Math.max(1, Math.ceil(stringWidth(line, { countAnsiEscapeCodes: true }) / columns));
    //     }
    // }

    get isEnabled() {
        return this.#isEnabled && !this.#isSilent;
    }

    set isEnabled(value) {
        if (typeof value !== 'boolean') {
            throw new TypeError('The `isEnabled` option must be a boolean');
        }

        this.#isEnabled = value;
    }

    get isSilent() {
        return this.#isSilent;
    }

    set isSilent(value) {
        if (typeof value !== 'boolean') {
            throw new TypeError('The `isSilent` option must be a boolean');
        }

        this.#isSilent = value;
    }

    // frame() {
    //     const { frames } = this.#spinner;
    //     let frame = frames[this.#frameIndex];

    //     if (this.color) {
    //         frame = chalk[this.color](frame);
    //     }

    //     this.#frameIndex = ++this.#frameIndex % frames.length;
    //     const fullPrefixText = (typeof this.#prefixText === 'string' && this.#prefixText !== '') ? this.#prefixText + ' ' : '';
    //     const fullText = typeof this.text === 'string' ? ' ' + this.text : '';
    //     const fullSuffixText = (typeof this.#suffixText === 'string' && this.#suffixText !== '') ? ' ' + this.#suffixText : '';

    //     return fullPrefixText + frame + fullText + fullSuffixText;
    // }

    clear() {
        if (!this.#isEnabled || !this.binding.isTty) {
            return this;
        }

        this.binding.clear();
        return this;

    }

    // render() {
    //     if (this.#isSilent) {
    //         return this;
    //     }

    //     this.clear();
    //     this.binding.pureWrite(this.frame());
    //     this.#linesToClear = this.#lineCount;

    //     return this;
    // }

    start(text?: string) {
        if (text) {
            this.text = text;
        }

        if (this.#isSilent) {
            return this;
        }

        if (!this.#isEnabled) {
            if (this.text) {
                this.binding.pureWrite(`- ${this.text}\n`);
            }

            return this;
        }

        if (this.isSpinning) {
            return this;
        }

        if (this.#options.discardStdin && process.stdin.isTTY) {
            this.#isDiscardingStdin = true;
            // stdinDiscarder.start();
        }

        this.binding.run();

        return this;
    }

    stop() {
        if (!this.#isEnabled) {
            return this;
        }

        // clearInterval(this.#id);
        this.binding.stop();
        this.clear();
        // if (this.#options.hideCursor) {
        //     cliCursor.show(this.#stream);
        // }

        if (this.#options.discardStdin && process.stdin.isTTY && this.#isDiscardingStdin) {
            // stdinDiscarder.stop();
            this.#isDiscardingStdin = false;
        }

        return this;
    }

    succeed(text: string = '') {
        return this.stopAndPersist({ symbol: logSymbols.success, text });
    }

    fail(text: string = '') {
        return this.stopAndPersist({ symbol: logSymbols.error, text });
    }

    warn(text: string = '') {
        return this.stopAndPersist({ symbol: logSymbols.warning, text });
    }

    info(text: string = '') {
        return this.stopAndPersist({ symbol: logSymbols.info, text });
    }

    stopAndPersist(options: PersistOptions = {}) {
        if (this.#isSilent) {
            return this;
        }

        const prefixText = options.prefixText ?? this.#prefixText;
        const fullPrefixText = this.#getFullPrefixText(prefixText, ' ');

        const symbolText = options.symbol ?? ' ';

        const text = options.text ?? this.text;
        const fullText = (typeof text === 'string') ? ' ' + text : '';

        const suffixText = options.suffixText ?? this.#suffixText;
        const fullSuffixText = this.#getFullSuffixText(suffixText, ' ');

        const textToWrite = fullPrefixText + symbolText + fullText + fullSuffixText + '\n';

        this.stop();
        this.binding.pureWrite(textToWrite);

        return this;
    }
}

export default function ora(options: Options) {
    return new Ora(options);
}

export async function oraPromise<T>(action: Promise<T> | ((spinner: Ora) => Promise<T>), options: PromiseOptions<T>) {
    const actionIsFunction = typeof action === 'function';
    const actionIsPromise = typeof (action as Promise<T>).then === 'function';

    if (!actionIsFunction && !actionIsPromise) {
        throw new TypeError('Parameter `action` must be a Function or a Promise');
    }

    const { successText, failText } = typeof options === 'object'
        ? options
        : { successText: undefined, failText: undefined };

    const spinner = ora(options).start();

    try {
        const promise = actionIsFunction ? action(spinner) : action;
        const result = await promise;

        spinner.succeed(
            successText === undefined
                ? undefined
                : (typeof successText === 'string' ? successText : successText(result)),
        );

        return result;
    } catch (error: any) {
        spinner.fail(
            failText === undefined
                ? undefined
                : (typeof failText === 'string' ? failText : failText(error)),
        );

        throw error;
    }
}

export { default as spinners } from 'cli-spinners';