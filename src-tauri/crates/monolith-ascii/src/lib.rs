pub mod error;
pub mod listener;
pub mod parser;

pub use error::ascii_error::AsciiError;

use std::sync::mpsc::{self, Receiver};
use std::thread;

use monolith_domain::CementingRecord;
use serialport::SerialPort;

use crate::listener::ascii_listener::start_ascii_listener;
use crate::parser::ascii_parser::parse_line;

pub fn start_ascii_stream(port: Box<dyn SerialPort>) -> Receiver<CementingRecord> {
    let raw_rx = start_ascii_listener(port);
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for message in raw_rx {
            match message {
                Ok(line) => {
                    if let Ok(record) = parse_line(&line) {
                        if tx.send(record).is_err() {
                            return;
                        }
                    }
                    // ошибки парсинга игнорируем
                }
                Err(_) => {
                    // ошибка чтения порта — завершаем поток
                    return;
                }
            }
        }
    });

    rx
}
