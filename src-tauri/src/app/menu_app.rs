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
        .text("crio", "Crio")
        .text("OWEN 210", "OWEN 210")
        .build()?;

           //TODO добавление меню Агрегат
    let agregat_menu = SubmenuBuilder::new(app, "Агрегат")
            .text("agregat_CA", "Агрегат ЦA")
            .text("azotka", "Азотка")
            .build()?;


    //TODO Графики - пределы осей
    let chart_menu = SubmenuBuilder::new(app, "Масштаб")
        .text("axis_auto", "Авторасширение осей")
        .text("axis_manual", "Ручные пределы осей")
        .build()?;
        // добавление меню редактора
    let editor_menu = SubmenuBuilder::new(app, "Редактор")
        .text("editor_job", "Редактор работ")
        .text("editor_charts", "Редактор графиков")
        .build()?;
        //TODO добавление меню доп. функций
    let dop_functions_menu = SubmenuBuilder::new(app, "Доп. функции")
        .text("dop_functions_1", "Функция 1")
        .text("dop_functions_2", "Функция 2")
        .text("dop_functions_3", "Функция 3")
        .build()?;
 
    let about_menu = SubmenuBuilder::new(app, "О программе")
        .text("about_app", "О программе")
        .build()?;

    let menu = MenuBuilder::new(app)
        .items(&[&file_menu, &connection_menu, &agregat_menu, &chart_menu, &editor_menu, &dop_functions_menu, &about_menu])
        .build()?;

    app.set_menu(menu)?;

    Ok(())
}
