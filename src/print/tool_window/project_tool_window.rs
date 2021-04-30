use druid::widget::{Flex, Label, Scroll, SizedBox};
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    LocalizedString, Menu, MenuItem, MouseEvent, PaintCtx, Size, UpdateCtx, Widget, WidgetExt,
};

use crate::app_command::print_command;
use crate::app_state::AppState;
use crate::components::icon_button::IconButton;
use crate::components::tree::Tree;
use crate::model::file_tree::FileEntry;

pub struct ProjectToolWindow {
    inner: Box<dyn Widget<AppState>>,
}

impl ProjectToolWindow {
    pub fn new() -> ProjectToolWindow {
        ProjectToolWindow {
            inner: SizedBox::empty().boxed(),
        }
    }

    fn rebuild_inner(&mut self, data: &AppState) {
        let mut flex = Flex::column();

        if data.current_dir.is_some() {
            let scroll = Scroll::new(Tree::new(|t: &FileEntry| {
                // todo: different for dir & file;
                return IconButton::from_label(
                    Label::new(t.name.as_str())
                        .with_text_color(crate::theme::BASIC_TEXT_COLOR)
                        .with_text_size(crate::theme::BASIC_TEXT_SIZE),
                )
                .on_click(|ctx, data: &mut FileEntry, _env| {
                    if !data.is_dir {
                        ctx.submit_command(print_command::SET_FILE.with(data.to_owned()));
                    }
                });
            }));
            flex.add_child(scroll);
        }

        let flex = flex
            .background(crate::theme::SIDEBAR_BACKGROUND)
            .expand_height()
            .lens(AppState::entry);

        if data.params.debug_layout {
            self.inner = flex.debug_paint_layout().boxed()
        } else {
            self.inner = flex.boxed();
        }
    }

    fn send_mouse(
        &mut self,
        ctx: &mut EventCtx,
        _data: &mut AppState,
        _env: &Env,
        mouse_event: &MouseEvent,
    ) {
        if !mouse_event.button.is_right() {
            return;
        }
        let menu: Menu<AppState> = Menu::empty().entry(
            MenuItem::new(LocalizedString::new("menu-item-reload").with_placeholder("Reload"))
                .command(print_command::RELOAD_DIR),
        );

        ctx.show_context_menu(menu, mouse_event.window_pos);
    }
}

#[allow(unused_variables)]
impl Widget<AppState> for ProjectToolWindow {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::MouseDown(m) => self.send_mouse(ctx, data, env, m),
            _ => {}
        }
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.rebuild_inner(data);
        }
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        if !old_data.current_dir.same(&data.current_dir) {
            self.rebuild_inner(data);
            ctx.children_changed();
        } else {
            self.inner.update(ctx, old_data, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.inner.paint(ctx, data, env);
    }
}
