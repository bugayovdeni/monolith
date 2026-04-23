use monolith_ascii::start_ascii_stream;
use monolith_serial::port::open_port::open_port;

#[tauri::command]
pub fn start_ascii_stream_command(port_name: String) -> Result<(), String> {
    //FIXME Дебаг - удалить
    println!("start_ascii_stream_command called, port = {}", port_name);

    let port = open_port(&port_name).map_err(|e| e.to_string())?;
    let rx = start_ascii_stream(port);

    std::thread::spawn(move || {
        for record in rx {
            println!("ASCII RECORD: {:?}", record);
        }
    });

    Ok(())
}
