use std::{
  error::Error,
  sync::{LazyLock, atomic::AtomicBool},
  time::Duration,
};

use crossbeam_queue::SegQueue;
use interprocess::local_socket::{GenericNamespaced, Stream, ToNsName, traits::Stream as _};
use shared_types::{EncodeError, Message, config::hook::HookLoggingConfig};

pub(crate) use macros::*;

static MSG_QUEUE: LazyLock<SegQueue<Message>> = LazyLock::new(Default::default);
static LOGGING_ENABLED: AtomicBool = AtomicBool::new(true);
static LOG_STDERR: AtomicBool = AtomicBool::new(false);

pub fn init_logging(config: HookLoggingConfig) {
  match config {
    HookLoggingConfig::Ipc(socket_name) => {
      let ns_name = socket_name.to_ns_name::<GenericNamespaced>().unwrap();
      let mut stream = Stream::connect(ns_name).unwrap();
      std::thread::spawn(move || {
        loop {
          if let Some(message) = MSG_QUEUE.pop()
            && let Err(err) = message.send(&mut stream)
          {
            eprintln!("{err:?}");
            if matches!(err, EncodeError::Io { .. } | EncodeError::UnexpectedEnd) {
              return;
            }
          }
        }
      });
    }
    HookLoggingConfig::Stderr => {
      LOG_STDERR.store(true, std::sync::atomic::Ordering::Relaxed);
    }
    HookLoggingConfig::None => LOGGING_ENABLED.store(false, std::sync::atomic::Ordering::Relaxed),
  }
}

pub fn log(msg: Message) {
  if LOG_STDERR.load(std::sync::atomic::Ordering::Relaxed) {
    eprintln!("{msg}")
  } else if LOGGING_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
    MSG_QUEUE.push(msg);
  } else if let Message::Error(err) = msg {
    panic!("{err}")
  }
}

pub fn log_lossy(msg: Message) {
  log(msg);
}

pub fn log_info(msg: impl ToString) {
  log(Message::DebugInfo(msg.to_string()));
}

#[track_caller]
pub fn log_debug(msg: impl std::fmt::Debug) {
  let location = std::panic::Location::caller();
  log(Message::DebugInfo(format!(
    "Debug | {}:{} = {msg:?}",
    location.file(),
    location.line()
  )));
}

pub fn log_error(err: impl Error) {
  log(Message::Error(err.to_string()));
}

mod macros {
  macro_rules! trace {
    ($($tt:tt)*) => {
      if let Err(err) = (|| {
        $($tt)*
        Result::<_, shared_types::HookError>::Ok(())
      })() {
        crate::log::log_lossy(shared_types::Message::Error(err.to_string()))
      }
    };
  }

  macro_rules! trace_expr {
    ($default:expr, $($tt:tt)*) => {
      {
        #[allow(clippy::redundant_closure_call)]
        let res: Result::<_, shared_types::HookError> = (|| {
          $($tt)*
        })();
        match res {
          Ok(val) => val,
          Err(err) => {
            crate::log::log_lossy(shared_types::Message::Error(err.to_string()));
            return $default
          }
        }
      }
    };
  }

  macro_rules! logfmt_dbg {
    ($fmt_str:literal$($tt:tt)*) => {
      crate::log::log(shared_types::Message::DebugInfo(format!(concat!("Debug | ", file!(), ":", line!(), " = ", $fmt_str) $($tt)*)))
    };
  }

  pub(crate) use {logfmt_dbg, trace, trace_expr};
}
