use serialport::available_ports;

pub fn scan() -> Vec<String> {
    match available_ports() {
        Ok(ports) => ports.into_iter().map(|p| p.port_name).collect(),

        Err(e) => {
            println!("Error scanning ports: {:?}", e);
            vec![]
        }
    }
}
