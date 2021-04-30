use crate::app_state::AppState;
use druid::{
    commands, platform_menus, Data, Env, FileDialogOptions, LocalizedString, Menu, MenuItem,
    SysMods, WindowId,
};

#[allow(unused_assignments)]
pub fn make_menu(_: Option<WindowId>, _state: &AppState, _: &Env) -> Menu<AppState> {
    let mut menu = Menu::empty();
    #[cfg(target_os = "macos")]
    {
        menu = menu.entry(platform_menus::mac::application::default());
    }

    menu.entry(file_menu())
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

