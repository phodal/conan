#[allow(unused_imports)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

use druid::{AppLauncher, Color, UnitPoint, WindowDesc};
use druid::widget::{Flex, Label, WidgetExt};
use druid::widget::prelude::*;

use app_state::AppState;
use print::menu;
use print::text_edit_view::TextEditView;
pub use support::line;

use crate::app_delegate::Delegate;
use crate::app_state::Workspace;
use crate::components::icon_button::IconButton;
use crate::print::ProjectToolWindow;
use crate::support::directory;

use self::print::bar_support::text_count;

pub mod app_command;
pub mod app_delegate;
pub mod app_state;
pub mod components;
pub mod file_manager;
pub mod model;
pub mod print;
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

pub fn main() {
    setup_log();

    let title = "Conan";

    let main_window = WindowDesc::new(make_ui())
        .window_size((1024., 768.))
        .with_min_size((1024., 768.))
        .menu(menu::make_menu)
        .title(title);


    let mut init_state = directory::read_config();
    init_state.setup_workspace();

    AppLauncher::with_window(main_window)
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
