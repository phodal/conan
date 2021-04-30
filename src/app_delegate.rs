use crate::app_command::print_command;
use crate::app_state::{AppState, Workspace};
use crate::components::modal_host::ModalHost;
use druid::widget::{Flex, Label};
use druid::{AppDelegate, Command, DelegateCtx, Env, FileInfo, Handled, Target, Widget, WidgetExt};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    #[rustfmt::skip]
    fn command<'a>(&mut self, ctx: &mut DelegateCtx<'a>, _target: Target, cmd: &Command, data: &mut AppState, _env: &Env, ) -> Handled {
        if let Some(info) = cmd.get(print_command::SET_FILE) {
            let path = PathBuf::from(info.path.as_str());
            log::info!("open file: {:?}", path.display());
            data.open_file(path);
            return Handled::Yes;
        } else if cmd.is(druid::commands::SAVE_FILE) {
            return Delegate::save_file(data);
        } else if cmd.is(print_command::RELOAD_DIR) {
            data.set_dir(data.current_dir.as_ref().unwrap().to_path_buf());
            return Handled::Yes;
        } else if cmd.is(druid::commands::SHOW_ABOUT) {
            let host = ModalHost::new(Delegate::paint_preferences());
            host.lens(AppState::workspace);
            return Handled::Yes;
        } else if let Some(info) = cmd.get(druid::commands::OPEN_FILE) {
            return Delegate::open_file(ctx, data, info);
        }

        Handled::No
    }
}

impl Delegate {
    fn open_file(ctx: &mut DelegateCtx, state: &mut AppState, info: &FileInfo) -> Handled {
        if info.path().is_dir() {
            state.set_dir(info.path().to_owned());
            ctx.submit_command(print_command::OPEN);
            return Handled::Yes;
        }

        if let Ok(typ) = infer::get_from_path(info.path()) {
            if let Some(_file_type) = typ {
                if let Some(parent) = info.path().parent() {
                    state.set_dir(Some(parent.to_owned()));
                }

                state.open_file(info.path().to_owned());
                ctx.submit_command(print_command::OPEN);
                return Handled::Yes;
            }
        };

        log::info!("under type: {:?}", info);
        return Handled::No;
    }

    fn save_file(data: &mut AppState) -> Handled {
        let file_path;
        match &data.current_file {
            None => return Handled::Yes,
            Some(path) => {
                file_path = path;
            }
        };

        let buf = file_path.to_path_buf();

        if data.workspace.input_text == data.workspace.origin_text {
            return Handled::Yes;
        }

        let mut ifile = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&buf)
            .expect("unable to open file");

        let result = ifile.write_all(data.text().as_bytes());

        match result {
            Ok(_) => log::info!("save file: {:?}", buf),
            Err(e) => log::info!("Failed to write data: {}", { e }),
        }

        return Handled::Yes;
    }

    fn paint_preferences() -> impl Widget<Workspace> {
        let flex = Flex::column()
            .with_child(Label::new("preferences").with_text_color(crate::theme::BASIC_TEXT_COLOR))
            .with_default_spacer();

        return flex;
    }
}
