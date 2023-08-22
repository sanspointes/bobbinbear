import { RingBuffer } from "ring-buffer-ts";
/**
 * Logging helpers to supress console.
 */
enum LogLevel {
  debug = 0,
  log = 1,
  info = 2,
  warn = 3,
  error = 4,
}

type LogEntry = {
  level: LogLevel;
  label?: string;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  message: any[];
};

// Hash is defined in vite.config.ts
// so that we can version all error reports.
declare const __COMMIT_HASH__: string;
/**
 * Logging helper that stores warnings / errors for error tracking.
 *
 * Not currently in use.
 */
class Logger {
  private logLevel: LogLevel = LogLevel.info;
  private storedLevel: LogLevel = LogLevel.debug;
  private logs: RingBuffer<LogEntry>;

  private currentLabel?: string;
  constructor(storedLength: number) {
    this.logs = new RingBuffer(storedLength);
  }

  getStored(): LogEntry[] {
    return this.logs.toArray();
  }
  getCommitHash(): string {
    return __COMMIT_HASH__;
  }

  setLogLevel(level: "debug" | "log" | "info" | "warn" | "error") {
    this.logLevel = LogLevel[level];
  }
  setStoredLevel(level: "debug" | "log" | "info" | "warn" | "error") {
    this.storedLevel = LogLevel[level];
  }

  debug(...args: unknown[]) {
    if (LogLevel.debug >= this.logLevel) console.debug("EJX_EDITOR: ", ...args);
    if (LogLevel.debug >= this.storedLevel) {
      this.logs.add({
        level: LogLevel.debug,
        message: args,
        label: this.currentLabel,
      });
    }
  }

  log(...args: unknown[]) {
    if (LogLevel.log >= this.logLevel) console.log("EJX_EDITOR: ", ...args);
    if (LogLevel.log >= this.storedLevel) {
      this.logs.add({
        level: LogLevel.log,
        message: args,
        label: this.currentLabel,
      });
    }
  }

  info(...args: unknown[]) {
    if (LogLevel.info >= this.logLevel) console.info("EJX_EDITOR: ", ...args);
    if (LogLevel.info >= this.storedLevel) {
      this.logs.add({
        level: LogLevel.info,
        message: args,
        label: this.currentLabel,
      });
    }
  }

  warn(...args: unknown[]) {
    if (LogLevel.warn >= this.logLevel) console.warn("EJX_EDITOR: ", ...args);
    if (LogLevel.warn >= this.storedLevel) {
      this.logs.add({
        level: LogLevel.warn,
        message: args,
        label: this.currentLabel,
      });
    }
  }

  error(...args: unknown[]) {
    if (LogLevel.error >= this.logLevel) console.error("EJX_EDITOR: ", ...args);
    if (LogLevel.warn >= this.storedLevel) {
      this.logs.add({
        level: LogLevel.warn,
        message: args,
        label: this.currentLabel,
      });
    }
  }

  group(label: string) {
    this.currentLabel = label;
    console.group(label);
  }

  groupEnd() {
    console.groupEnd();
  }
}

export const logger = new Logger(500);
logger.setLogLevel('debug');

// @ts-expect-error; TEMP,TODO: Remove this to make logger non,global 
window.logger = logger;
