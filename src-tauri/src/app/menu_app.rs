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
        .text("flecs", "Flecs_4-20")
        .build()?;

    //TODO Графики - пределы осей
    let chart_menu = SubmenuBuilder::new(app, "Графики")
        .text("axis_auto", "Авторасширение осей")
        .text("axis_manual", "Ручные пределы осей")
        .build()?;

    let about_menu = SubmenuBuilder::new(app, "О программе")
        .text("about_app", "О программе")
        .build()?;

    let menu = MenuBuilder::new(app)
        .items(&[&file_menu, &connection_menu, &chart_menu, &about_menu])
        .build()?;

    app.set_menu(menu)?;

    Ok(())
}
