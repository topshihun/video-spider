use std::path::PathBuf;
use std::fs::read_dir;
use dirs::config_dir;

pub struct LuaFile {
    pub name: String,
    pub path: PathBuf,
}

pub fn get_lua_files() -> Vec<LuaFile> {
    let mut config_path = config_dir().expect("Can't find config directory.");
    config_path.push(env!("CARGO_PKG_NAME"));
    std::fs::create_dir_all(&config_path).expect("Failed to create config path");
    let mut lua_files: Vec<LuaFile> = Vec::new();
    for entry in read_dir(config_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && let Some(ext) = path.extension() {
            if ext == "lua" {
                lua_files.push(LuaFile {
                    name: path.file_name().unwrap().to_string_lossy().into_owned(),
                    path: path,
                });
            }
        }
    }
    lua_files
}
