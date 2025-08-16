use dirs::config_dir;
use std::fs::read_dir;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct LuaFile {
    pub name: String,
    pub path: PathBuf,
}

impl PartialEq for LuaFile {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for LuaFile {}

impl Hash for LuaFile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.as_bytes().hash(state);
    }
}

pub fn get_lua_files() -> Vec<LuaFile> {
    let mut config_path = config_dir().expect("Can't find config directory.");
    config_path.push("videospider");
    std::fs::create_dir_all(&config_path).expect("Failed to create config path");
    let mut lua_files: Vec<LuaFile> = Vec::new();
    for entry in read_dir(config_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file()
            && let Some(ext) = path.extension()
            && ext == "lua"
        {
            lua_files.push(LuaFile {
                name: path.file_name().unwrap().to_string_lossy().into_owned(),
                path,
            });
        }
    }
    lua_files
}
