#[allow(unused_imports)]
#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_json;
extern crate xi_core_lib;
extern crate xi_rpc;
extern crate xi_trace;

use std::sync::{Arc, Mutex};
use std::thread;

use druid::widget::prelude::*;
use druid::widget::{Flex, Label, WidgetExt};
use druid::{AppLauncher, Color, Target, UnitPoint, WindowDesc};
use log::*;

use app_state::AppState;
use print::menu;
use print::text_edit_view::TextEditView;
use rpc::client::Client;
pub use rpc::structs::{
    Alert, AvailableLanguages, AvailablePlugins, AvailableThemes, ConfigChanged, ConfigChanges,
    FindStatus, LanguageChanged, Line, MeasureWidth, ModifySelection, Operation, OperationType,
    PluginStarted, PluginStopped, Position, Query, ReplaceStatus, ScrollTo, Status, Style,
    StyleDef, ThemeChanged, ThemeSettings, Update, UpdateCmds, ViewId,
};
pub use support::line;

use crate::app_command::print_command;
use crate::app_delegate::Delegate;
use crate::app_state::Workspace;
use crate::components::icon_button::IconButton;
use crate::print::edit_view::EditView;
use crate::print::ProjectToolWindow;
use crate::support::directory;

use self::print::bar_support::text_count;

pub mod app_command;
pub mod app_delegate;
pub mod app_state;
pub mod components;
pub mod file_manager;
pub mod linecache;
pub mod model;
pub mod print;
pub mod rpc;
pub mod support;
pub mod theme;

fn navigation_bar() -> impl Widget<AppState> {
    let label = Label::new(|workspace: &Workspace, _env: &Env| workspace.relative_path())
        .with_text_color(Color::BLACK);
    Flex::row()
        .with_child(label)
        .padding(10.0)
        .expand_width()
        .lens(AppState::workspace)
        .background(line::hline())
        .align_horizontal(UnitPoint::LEFT)
}

fn status_bar() -> impl Widget<AppState> {
    let label = Label::new(|data: &Workspace, _env: &Env| {
        return text_count::count(&data.input_text).to_string();
    })
    .with_text_color(Color::BLACK);

    Flex::row()
        .with_default_spacer()
        .with_flex_child(Label::new("words: ").with_text_color(Color::BLACK), 1.0)
        .with_default_spacer()
        .with_flex_child(label, 1.0)
        .with_default_spacer()
        .lens(AppState::workspace)
        .padding(5.0)
        .align_horizontal(UnitPoint::LEFT)
}

fn bottom_tool_window() -> impl Widget<AppState> {
    let text = "Run";
    let label = Label::new(text).with_text_color(Color::BLACK);
    let button = IconButton::from_label(label);
    Flex::row()
        .with_default_spacer()
        .with_flex_child(button, 1.0)
        .lens(AppState::params)
        .background(line::hline())
}

fn center() -> impl Widget<AppState> {
    Flex::row()
        .with_child(ProjectToolWindow::new())
        .with_default_spacer()
        .with_flex_child(TextEditView::new().center(), 1.0)
        .with_default_spacer()
        .with_flex_child(EditView::new().center(), 1.0)
        .padding(1.0)
        .expand_height()
        .expand_width()
        .background(line::hline())
}

fn make_ui() -> impl Widget<AppState> {
    Flex::column()
        .with_child(navigation_bar())
        .with_flex_child(center(), 1.0)
        .with_child(bottom_tool_window())
        .with_child(status_bar())
        .background(crate::theme::BACKGROUND_COLOR)
}

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

pub fn main() {
    setup_log();

    let title = "Print UI";

    let main_window = WindowDesc::new(make_ui())
        .window_size((1024., 768.))
        .with_min_size((1024., 768.))
        .menu(menu::make_menu)
        .title(title);

    let (client, rpc_receiver) = Client::new();

    let launcher = AppLauncher::with_window(main_window);
    let handler = launcher.get_external_handle();

    thread::spawn(move || loop {
        match rpc_receiver.recv() {
            Ok(operations) => {
                handler
                    .submit_command(print_command::XI_EVENT, Box::new(operations), Target::Auto)
                    .expect("Failed to send command");
            }
            Err(err) => {
                error!("error: {:?}", err);
                panic!("{:?}", err);
            }
        }
    });

    let mut init = directory::read_config();
    let client = Arc::new(Mutex::new(client));
    client
        .lock()
        .unwrap()
        .client_started(Some(&"config".to_string()), Some(&"config".to_string()));

    if init.current_file.is_some() {
        let file = init.current_file.clone().as_ref().unwrap().to_owned();
        let path_str = format!("{}", file.display());
        client.lock().unwrap().new_view(path_str, move |_| {});
    }

    if !init.theme_name.is_empty() {
        client.lock().unwrap().send_notification(
            "set_theme",
            &json!({ "theme_name": init.theme_name.clone() }),
        );
    }

    client.lock().unwrap().modify_user_config_domain(
        "general",
        &json!({
            "tab_size": 4,
            "autodetect_whitespace": true,
            "translate_tabs_to_spaces": true,
            "font_face": "Inconsolata",
            "font_size": 14.0,
            "use_tab_stops": true,
            "word_wrap": false,
            "line_ending": LINE_ENDING,
        }),
    );

    init.core = client;

    let state = Arc::new(Mutex::new(init));
    let mut init_state = state.lock().unwrap().to_owned();

    init_state.setup_workspace();

    launcher
        .delegate(Delegate::default())
        .configure_env(|env, _| theme::configure_env(env))
        .launch(init_state)
        .expect("Failed to launch application");
}

fn setup_log() {
    use tracing_subscriber::prelude::*;
    let filter_layer = tracing_subscriber::filter::LevelFilter::DEBUG;
    let fmt_layer = tracing_subscriber::fmt::layer()
        // Display target (eg "my_crate::some_mod::submod") with logs
        .with_target(true);

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}
