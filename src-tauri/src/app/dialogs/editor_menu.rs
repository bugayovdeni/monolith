use tauri::AppHandle;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

///
/// ## Обработчик \
/// ### Редактор
///
///
pub fn editor(app: &AppHandle) {
    let _version = app.package_info().version.to_string();
   
   // TODO Исправить
   let message = "Редактор в разработке".to_string();

    app.dialog()
        .message(message)
        .kind(MessageDialogKind::Info)
        .title("Редактор")
        .buttons(MessageDialogButtons::OkCustom("ОК".to_string()))
        .show(|result| match result {
            true => (),  // do something,
            false => (), // do something,
        });
}
