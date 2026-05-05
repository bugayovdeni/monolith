use serialport::SerialPort;
use std::time::Duration;

use crate::error::serial_error::SerialError;

pub fn open_port(port_name: &str) -> Result<Box<dyn SerialPort>, SerialError> {
    serialport::new(port_name, 9600)
        .timeout(Duration::from_millis(1000))
        .open()
        .map_err(|e| SerialError::OpenError(e.to_string()))
}
