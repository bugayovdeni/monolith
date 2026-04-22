//! monolith-serial
//!
//! Общий crate для serial transport/discovery.
//! Не зависит от Tauri.
//! Не содержит протокольную логику ASCII/RTU.
//!
//! Ответственность:
//! - scan serial ports
//! - serial config
//! - open/close serial port
//! - low-level read/write abstractions

pub mod error;
pub mod port;
pub mod scanner;
