use crate::app::close as close_app;
use tauri::{Manager, WebviewWindow};

pub fn close_event(main_window: &WebviewWindow) {
    let window_clone = main_window.clone();

    main_window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            // ВАЖНО: отменяем стандартное закрытие, иначе окно сдохнет до того, как сохраняться данные
            api.prevent_close();
            //FIXME Удалить print
            println!("Попытка закрытия перехвачена. Делаем свои дела...");

            //TODO  Получаем событие окна
            let app_handle = window_clone.app_handle().clone();

            //TODO Перехватчик закрытия окна
            close_app(&app_handle);
        }
    });
}
