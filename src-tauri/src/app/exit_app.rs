use tauri::AppHandle;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

/// Перехватчик закрытия окна \
/// ## Закрытие программы  
/// с подтверждением в окне диалог \
/// \
/// *принимает параметр*
///
/// `AppHandle`
///
pub fn close(app_handle: &AppHandle) {
    let app_handle = app_handle.clone();
    app_handle
        .dialog()
        .message("Подтверждаете Выход?")
        .title("Выход из Программы")
        .buttons(MessageDialogButtons::OkCancelCustom(
            "Да".to_string(),
            "Нет".to_string(),
        ))
        .show(move |result| {
            if result {
                app_handle.exit(0);
            }
        });
}
