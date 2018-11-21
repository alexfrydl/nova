// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate log;

pub use log::SetLoggerError as SetAsDefaultError;

use super::{Color, ContextBuilder, Level, LevelFilter, PrettyLevel};
use chrono::{Datelike, Timelike};
use std::fmt;
use std::io::{self, Write};

/// Writes formatted log messages to stdout with optional contextual
/// information.
#[derive(Debug)]
pub struct Logger {
  out: io::Stdout,
  /// Name describing the source of the messages. For example, the standard log
  /// macros use the current module path as the source.
  pub source: String,
  /// The highest level of logging that will be printed.
  pub max_level: LevelFilter,
}

impl Logger {
  /// Creates a new logger with the given source name.
  pub fn new(source: impl Into<String>) -> Self {
    Logger {
      out: io::stdout(),
      source: source.into(),
      max_level: LevelFilter::Trace,
    }
  }

  /// Sets this logger as the global default implementation for the standard log
  /// macros.
  pub fn set_as_default(&self) -> Result<(), SetAsDefaultError> {
    log::set_max_level(self.max_level);
    log::set_boxed_logger(Box::new(self.clone()))
  }

  /// Creates a new logger with a different source name but the same settings.
  pub fn with_source(&self, name: impl Into<String>) -> Self {
    Logger {
      out: io::stdout(),
      source: name.into(),
      max_level: self.max_level,
    }
  }

  /// Outputs a trace level message.
  pub fn trace(&self, msg: impl fmt::Display) -> ContextBuilder {
    self.log(&self.source, Level::Trace, msg)
  }

  /// Outputs a debug level message.
  pub fn debug(&self, msg: impl fmt::Display) -> ContextBuilder {
    self.log(&self.source, Level::Debug, msg)
  }

  /// Outputs an info level message.
  pub fn info(&self, msg: impl fmt::Display) -> ContextBuilder {
    self.log(&self.source, Level::Info, msg)
  }

  /// Outputs a warning level message.
  pub fn warn(&self, msg: impl fmt::Display) -> ContextBuilder {
    self.log(&self.source, Level::Warn, msg)
  }

  /// Outputs an error level message.
  pub fn error(&self, msg: impl fmt::Display) -> ContextBuilder {
    self.log(&self.source, Level::Error, msg)
  }

  /// Outputs a message with the given source name and level.
  fn log<'a>(&'a self, source: &str, level: Level, msg: impl fmt::Display) -> ContextBuilder<'a> {
    // Ignore messages above the maximum.
    if level > self.max_level {
      return Default::default();
    }

    let time = chrono::Utc::now();
    let mut out = self.out.lock();

    // Output a timestamp and colorized level.
    write!(
      out,
      "{}{:04}-{:02}-{:02} {:02}:{:02}:{:02} {}",
      Color::BrightBlack,
      time.year(),
      time.month(),
      time.day(),
      time.hour(),
      time.minute(),
      time.second(),
      PrettyLevel(level),
    )
    .unwrap();

    // Output the source name in brackets.
    if !source.is_empty() {
      write!(out, " {}[{}]{}", Color::White, source, Color::Reset).unwrap();
    }

    // Output the message.
    write!(out, " {}{}{}", Color::BrightWhite, msg, Color::Reset).unwrap();

    // Return a context builder so the caller can add more information.
    ContextBuilder::new(out)
  }
}

// Implement `Default` to create a new logger with no source name.
impl Default for Logger {
  fn default() -> Self {
    Logger::new(String::default())
  }
}

// Implement `Clone` so that loggers can easily be shared.
impl Clone for Logger {
  fn clone(&self) -> Self {
    Logger {
      out: io::stdout(),
      source: self.source.clone(),
      max_level: self.max_level,
    }
  }
}

// Implement `log::Log` so a logger can be used for the standard log macros.
impl log::Log for Logger {
  fn enabled(&self, metadata: &log::Metadata) -> bool {
    metadata.level() <= self.max_level
  }

  fn log(&self, record: &log::Record) {
    let source = if self.source.is_empty() {
      record.module_path().unwrap_or("")
    } else {
      &self.source
    };

    self.log(source, record.level(), record.args());
  }

  fn flush(&self) {}
}
