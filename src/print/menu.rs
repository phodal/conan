use crate::app_command::print_command;
use crate::app_state::AppState;
use druid::{
    commands, platform_menus, Data, Env, FileDialogOptions, LocalizedString, Menu, MenuItem,
    SysMods, WindowId,
};

#[allow(unused_assignments)]
pub fn make_menu(_: Option<WindowId>, state: &AppState, _: &Env) -> Menu<AppState> {
    let mut menu = Menu::empty();
    #[cfg(target_os = "macos")]
    {
        menu = menu.entry(platform_menus::mac::application::default());
    }

    menu.entry(file_menu()).entry(view_menu(state)).rebuild_on(
        |old_data: &AppState, data: &AppState, _env| data.themes.len() != old_data.themes.len(),
    )
}

fn view_menu(state: &AppState) -> Menu<AppState> {
    Menu::new(LocalizedString::new("common-menu-view-menu")).entry(themes_menu(state))
}

fn file_menu<T: Data>() -> Menu<T> {
    let open_file = commands::SHOW_OPEN_PANEL.with(FileDialogOptions::new().select_directories());
    Menu::new(LocalizedString::new("common-menu-file-menu"))
        .entry(platform_menus::mac::file::new_file())
        .entry(
            MenuItem::new(LocalizedString::new("common-menu-file-open"))
                .command(open_file)
                .hotkey(SysMods::Cmd, "o"),
        )
        .entry(platform_menus::mac::file::save())
        .separator()
        .entry(platform_menus::mac::file::close())
}

fn themes_menu(state: &AppState) -> Menu<AppState> {
    let mut themes_menu: Menu<AppState> =
        Menu::new(LocalizedString::new("common-menu-themes-menu"));
    for theme in &state.themes {
        let string = theme.clone();
        themes_menu = themes_menu
            .entry(MenuItem::new(string.clone()).command(print_command::SET_THEME.with(string)));
    }

    themes_menu
}
