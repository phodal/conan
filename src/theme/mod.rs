use druid::{Color, Env, FontDescriptor, FontFamily, FontStyle, FontWeight, Key};

pub const SIDEBAR_BACKGROUND: Key<Color> = Key::new("print.sidebar-background");
pub const SIDEBAR_EDGE_STROKE: Key<Color> = Key::new("print.sidebar-edge-stroke");

pub const TOOL_WINDOW_COLOR: Key<Color> = Key::new("print.tool-window-color");

pub const FOREGROUND_LIGHT: Key<Color> = Key::new("print.theme.foreground_light");
pub const FOREGROUND_DARK: Key<Color> = Key::new("print.theme.foreground_dark");

pub const BACKGROUND_COLOR: Key<Color> = Key::new("print.theme.bg-color");
pub const BUTTON_DARK: Key<Color> = Key::new("print.theme.button-dark");
pub const BUTTON_LIGHT: Key<Color> = Key::new("print.theme.button-light");

pub const BASIC_TEXT_COLOR: Key<Color> = Key::new("print.theme.text-dark");

pub const BORDERED_WIDGET_HEIGHT: Key<f64> = Key::new("print.theme.button-light-height");
pub const BUTTON_BORDER_WIDTH: Key<f64> = Key::new("print.theme.button-border-width");
pub const BASIC_TEXT_SIZE: Key<f64> = Key::new("print.theme.basic-font-size");

pub const WRITING_FONT: Key<FontDescriptor> = Key::new("print.theme.writing");

#[rustfmt::skip]
pub fn configure_env(env: &mut Env) {
    env.set(druid::theme::BACKGROUND_LIGHT, Color::WHITE);
    env.set(druid::theme::CURSOR_COLOR, Color::BLACK);

    env.set(crate::theme::SIDEBAR_BACKGROUND, Color::from_hex_str("#fff").unwrap());
    env.set(crate::theme::BACKGROUND_COLOR,Color::from_hex_str("#e7e7e7").unwrap());
    env.set(crate::theme::TOOL_WINDOW_COLOR,Color::from_hex_str("#fff").unwrap());
    env.set(crate::theme::SIDEBAR_BACKGROUND,Color::from_hex_str("#fff").unwrap());
    env.set(crate::theme::SIDEBAR_EDGE_STROKE,Color::from_hex_str("#c7c7c7").unwrap());
    env.set(crate::theme::BUTTON_LIGHT,Color::from_hex_str("#e7e7e7").unwrap());
    env.set(crate::theme::BUTTON_DARK,Color::from_hex_str("#b9b9b9").unwrap());
    env.set(crate::theme::BASIC_TEXT_COLOR,Color::from_hex_str("#000").unwrap());
    env.set(crate::theme::FOREGROUND_LIGHT,Color::from_hex_str("#fff").unwrap());
    env.set(crate::theme::FOREGROUND_DARK,Color::from_hex_str("#000").unwrap());
    env.set(crate::theme::BORDERED_WIDGET_HEIGHT,   32.0);
    env.set(crate::theme::BUTTON_BORDER_WIDTH,    2.0);
    env.set(crate::theme::BASIC_TEXT_SIZE,   12.0);

    let family = FontFamily::new_unchecked("Microsoft Yahei");
    env.set(crate::theme::WRITING_FONT,   FontDescriptor::new(family)
        .with_style(FontStyle::Regular)
        .with_weight(FontWeight::LIGHT)
        .with_size(15.0));
}
