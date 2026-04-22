use serialport::available_ports;

pub fn scan_ports() -> Vec<String> {
    match available_ports() {
        Ok(ports) => ports.into_iter().map(|p| p.port_name).collect(),
        Err(e) => {
            eprintln!("Error scanning serial ports: {e:?}");
            vec![]
        }
    }
}
