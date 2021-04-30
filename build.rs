use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const THEME_PATH_FILE: &str = "theme_path.rs";
const DEFAULT_THEME_PATH: &str = "resources/default.theme";

fn main() {
    generate_theme_path();
}

fn generate_theme_path() {
    let user_path: Option<&str> = option_env!("RB_THEME_PATH");
    let theme_path = user_path.unwrap_or(DEFAULT_THEME_PATH);

    let gen_path = Path::new(&env::var("OUT_DIR").unwrap()).join(THEME_PATH_FILE);
    let mut file = BufWriter::new(File::create(&gen_path).unwrap());
    writeln!(
        &mut file,
        "static THEME_FILE_PATH: &str = \"{}\";",
        theme_path
    )
    .unwrap();
}
