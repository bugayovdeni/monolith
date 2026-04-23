use monolith_ascii::start_ascii_stream;
use monolith_serial::port::open_port::open_port;
use tauri::{AppHandle, Emitter};

#[tauri::command]
pub fn start_ascii_stream_command(app: AppHandle, port_name: String) -> Result<(), String> {
    println!(
        "[ASCII CMD] start_ascii_stream_command called, port = {}",
        port_name
    );

    let port = open_port(&port_name).map_err(|e| {
        let msg = e.to_string();
        println!("[ASCII CMD] open_port FAILED: {}", msg);
        msg
    })?;

    println!("[ASCII CMD] open_port OK: {}", port_name);

    let rx = start_ascii_stream(port);

    std::thread::spawn(move || {
        println!("[ASCII CMD] receiver thread started");

        for record in rx {
            let _ = app.emit("ascii-record", &record);
        }

        println!("[ASCII CMD] receiver thread finished");
    });

    Ok(())
}
