use tauri::menu::{MenuBuilder, SubmenuBuilder};
use tauri::App;

///
/// ## Настройка меню
///
/// *принимает параметр*\
///
/// `App`
///
pub fn setup_menu(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let file_menu = SubmenuBuilder::new(app, "Файл")
        .text("open", "Открыть")
        .text("save_as", "Сохранить как")
        .text("quit", "Выход")
        .build()?;

    let connection_menu = SubmenuBuilder::new(app, "Подключение")
        .text("flecs", "Flecs")
        .build()?;

    let about_menu = SubmenuBuilder::new(app, "О программе")
        .text("about_app", "О прграмме")
        .build()?;

    let menu = MenuBuilder::new(app)
        .items(&[&file_menu, &connection_menu, &about_menu])
        .build()?;

    app.set_menu(menu)?;

    Ok(())
}
