use tauri::AppHandle;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

///
/// ## Обработчик \
/// ### Агрегат
///
///Добавлен атрибут #[allow(dead_code)] для неиспользуемой функции agregat, чтобы подавить предупреждение.
#[allow(dead_code)]
pub fn agregat(app: &AppHandle) {
    let _version = app.package_info().version.to_string();
   
   // TODO Исправить
   let message = "... в разработке".to_string();

    app.dialog()
        .message(message)
        .kind(MessageDialogKind::Info)
        .title("Агрегат")
        .buttons(MessageDialogButtons::OkCustom("ОК".to_string()))
        .show(|result| match result {
            true => (),  // do something,
            false => (), // do something,
        });
}

///
/// ## Обработчик \
/// ### Агрегат ЦA
///
///
pub fn agregat_ca(app: &AppHandle) {
    let _version = app.package_info().version.to_string();
   
   // TODO Исправить
   let message = "Агрегат ЦA в разработке".to_string();

    app.dialog()
        .message(message)
        .kind(MessageDialogKind::Info)
        .title("Агрегат ЦA")
        .buttons(MessageDialogButtons::OkCustom("ОК".to_string()))
        .show(|result| match result {
            true => (),  // do something,
            false => (), // do something,
        });
}

///
/// ## Обработчик \
/// ### Азотка
///
///
pub fn azotka(app: &AppHandle) {
    let _version = app.package_info().version.to_string();
   
   // TODO Исправить
   let message = "Азотка в разработке".to_string();

    app.dialog()
        .message(message)
        .kind(MessageDialogKind::Info)
        .title("Азотка")
        .buttons(MessageDialogButtons::OkCustom("ОК".to_string()))
        .show(|result| match result {
            true => (),  // do something,
            false => (), // do something,
        });
}
