use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use monolith_ascii::start_ascii_stream;
use monolith_serial::port::open_port::open_port;
use tauri::{AppHandle, Emitter, Manager};

#[derive(Default)]
pub struct SerialControl {
    pub stop: Option<Arc<AtomicBool>>,
}

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

    let control = app.state::<Arc<Mutex<SerialControl>>>();
    let mut control = control
        .lock()
        .map_err(|_| "SerialControl lock poisoned".to_string())?;

    if let Some(old_stop) = &control.stop {
        old_stop.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    let stop = Arc::new(AtomicBool::new(false));
    control.stop = Some(stop.clone());

    let rx = start_ascii_stream(port, stop);
    drop(control);

    std::thread::spawn(move || {
        println!("[ASCII CMD] receiver thread started");

        for record in rx {
            let _ = app.emit("ascii-record", &record);
        }

        println!("[ASCII CMD] receiver thread finished");
    });

    Ok(())
}

#[tauri::command]
pub fn stop_serial(app: AppHandle) -> Result<(), String> {
    let control = app.state::<Arc<Mutex<SerialControl>>>();
    let mut control = control
        .lock()
        .map_err(|_| "SerialControl lock poisoned".to_string())?;

    if let Some(stop) = &control.stop {
        stop.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    control.stop = None;

    println!("[SERIAL] stopped");

    Ok(())
}
