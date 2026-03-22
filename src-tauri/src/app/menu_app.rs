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
        .text("quit", "Выход")
        .build()?;

    let menu = MenuBuilder::new(app).items(&[&file_menu]).build()?;

    app.set_menu(menu)?;

    Ok(())
}
