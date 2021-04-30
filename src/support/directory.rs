extern crate dirs;

use crate::app_state::AppState;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

pub fn save_config(state: &AppState) {
    let result = serde_json::to_string_pretty(&state);
    match result {
        Ok(str) => {
            let path = config_path().expect("lost home issue");
            if !path.exists() {
                let _file = File::create(&path).unwrap();
            }

            let result = fs::write(&path, str);

            match result {
                Ok(_) => log::info!("save config: {:?}", path),
                Err(e) => log::info!("failed to write data: {}", { e }),
            }
        }
        Err(err) => {
            log::info!("serialize config error: {:?}", err);
        }
    }
}

#[allow(unused_assignments)]
pub fn read_config() -> AppState {
    let mut app_state = AppState::default();
    let path = config_path().expect("lost home issue");
    let content;
    match fs::read_to_string(&path) {
        Ok(str) => {
            content = str;
        }
        Err(_) => {
            return app_state;
        }
    }

    match serde_json::from_str(&content) {
        Ok(state) => {
            app_state = state;
        }
        Err(_err) => {
            log::error!("error config: {}", content);
        }
    };
    return app_state;
}

pub fn config_path() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    let base = home.join(".print");
    if !&base.exists() {
        let _ = fs::create_dir_all(&base);
    }
    let config_path = base.join("print.json");
    Some(config_path)
}
