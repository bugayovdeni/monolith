use tauri::AppHandle;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

///
/// ## Обработчик \
/// ### Дополнительные функции
///
///
pub fn dop_functions(app: &AppHandle) {
    let _version = app.package_info().version.to_string();
   
   // TODO Исправить
   let message = "Доп. функция в разработке".to_string();

    app.dialog()
        .message(message)
        .kind(MessageDialogKind::Info)
        .title("Дополнительные функции")
        .buttons(MessageDialogButtons::OkCustom("ОК".to_string()))
        .show(|result| match result {
            true => (),  // do something,
            false => (), // do something,
        });
}
