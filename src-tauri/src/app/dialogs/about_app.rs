use tauri::AppHandle;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

///
/// ## Обработчик \
/// ### О Программе
///
///
pub fn about(app: &AppHandle) {
    let version = app.package_info().version.to_string();
    let message = format!(
        "Monolith\n\
        Версия: {} DEV\n\
        \n\
        \n\
        Лицензия: Тестировщик\n\
        \n\
        БУРСЕРВИС \n\
        © 2026",
        version
    );

    app.dialog()
        .message(message)
        .kind(MessageDialogKind::Info)
        .title("О программе")
        .buttons(MessageDialogButtons::OkCustom("Ясно".to_string()))
        .show(|result| match result {
            true => (),  // do something,
            false => (), // do something,
        });
}
