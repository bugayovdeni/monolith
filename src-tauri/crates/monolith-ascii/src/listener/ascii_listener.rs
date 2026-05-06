use std::io::ErrorKind;
use std::io::Read;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver};
use std::thread;

use serialport::SerialPort;

use crate::error::ascii_error::AsciiError;

pub fn start_ascii_listener(
    mut port: Box<dyn SerialPort>,
    stop: Arc<AtomicBool>,
) -> Receiver<Result<String, AsciiError>> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut buffer: Vec<u8> = Vec::new();
        let mut temp = [0u8; 1024];

        while !stop.load(Ordering::Relaxed) {
            match port.read(&mut temp) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        continue;
                    }

                    //FIXME Дебаг - удалить
                    // println!(
                    //     "[ASCII LISTENER] RAW TEXT: {:?}",
                    //     String::from_utf8_lossy(&temp[..bytes_read])
                    // );

                    buffer.extend_from_slice(&temp[..bytes_read]);

                    while let Some(line_bytes) = extract_line(&mut buffer) {
                        let line = String::from_utf8_lossy(&line_bytes).into_owned();

                        if line.trim().is_empty() {
                            continue;
                        }

                        if tx.send(Ok(line)).is_err() {
                            return;
                        }
                    }
                }
                Err(err) if err.kind() == ErrorKind::TimedOut => {
                    continue;
                }
                Err(err) => {
                    let _ = tx.send(Err(AsciiError::ReadError(err.to_string())));
                    return;
                }
            }
        }
    });

    rx
}

fn extract_line(buffer: &mut Vec<u8>) -> Option<Vec<u8>> {
    // Поддержка трёх вариантов разделителей строк: \r\n (Windows), \n (Unix), \r (старый Mac)
    // Приоритет: сначала ищем \r\n, затем \n, затем \r.
    let mut position = None;
    let mut delimiter_len = 0;

    // Поиск \r\n (два байта)
    if buffer.len() >= 2 {
        if let Some(idx) = buffer.windows(2).position(|w| w == b"\r\n") {
            position = Some(idx);
            delimiter_len = 2;
        }
    }

    // Если \r\n не найден, ищем \n
    if position.is_none() {
        if let Some(idx) = buffer.iter().position(|&b| b == b'\n') {
            position = Some(idx);
            delimiter_len = 1;
        }
    }

    // Если \n не найден, ищем \r
    if position.is_none() {
        if let Some(idx) = buffer.iter().position(|&b| b == b'\r') {
            position = Some(idx);
            delimiter_len = 1;
        }
    }

    // Если ни один разделитель не найден, строка неполная
    let position = position?;

    // Извлекаем строку до разделителя
    let line = buffer[..position].to_vec();
    // Удаляем строку вместе с разделителем из буфера
    buffer.drain(..position + delimiter_len);

    Some(line)
}

#[cfg(test)]
mod tests {
    use super::extract_line;

    #[test]
    fn extract_line_returns_one_complete_line() {
        let mut buffer = b"abc\r\n".to_vec();

        let line = extract_line(&mut buffer);

        assert_eq!(line, Some(b"abc".to_vec()));
        assert!(buffer.is_empty());
    }

    #[test]
    fn extract_line_keeps_tail_after_complete_line() {
        let mut buffer = b"abc\r\ndef".to_vec();

        let line = extract_line(&mut buffer);

        assert_eq!(line, Some(b"abc".to_vec()));
        assert_eq!(buffer, b"def".to_vec());
    }

    #[test]
    fn extract_line_returns_line_for_carriage_return() {
        // Тест проверяет, что одиночный \r считается разделителем
        let mut buffer = b"abc\r".to_vec();

        let line = extract_line(&mut buffer);

        assert_eq!(line, Some(b"abc".to_vec()));
        assert!(buffer.is_empty());
    }

    #[test]
    fn extract_line_returns_line_for_newline() {
        // Тест проверяет, что \n считается разделителем
        let mut buffer = b"xyz\n".to_vec();

        let line = extract_line(&mut buffer);

        assert_eq!(line, Some(b"xyz".to_vec()));
        assert!(buffer.is_empty());
    }

    #[test]
    fn extract_line_returns_none_for_no_delimiter() {
        // Тест проверяет, что если в буфере нет ни одного разделителя, возвращается None
        let mut buffer = b"abc".to_vec();

        let line = extract_line(&mut buffer);

        assert_eq!(line, None);
        assert_eq!(buffer, b"abc".to_vec());
    }

    #[test]
    fn extract_line_returns_empty_line() {
        let mut buffer = b"\r\n".to_vec();

        let line = extract_line(&mut buffer);

        assert_eq!(line, Some(Vec::new()));
        assert!(buffer.is_empty());
    }

    #[test]
    fn extract_line_can_extract_two_lines_sequentially() {
        let mut buffer = b"abc\r\ndef\r\n".to_vec();

        let first = extract_line(&mut buffer);
        let second = extract_line(&mut buffer);
        let third = extract_line(&mut buffer);

        assert_eq!(first, Some(b"abc".to_vec()));
        assert_eq!(second, Some(b"def".to_vec()));
        assert_eq!(third, None);
        assert!(buffer.is_empty());
    }
}
